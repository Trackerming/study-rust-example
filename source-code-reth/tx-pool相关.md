## 初始化transaction pool

### 初始化BlockchainProvider<DB, Tree>

- 依赖两个结构，前面提到的`blockchain_tree`和`ProviderFactory`用于读取数据库；
- 得到了header之后，得到`ChainInfoTracker`， 里面的结构如下，读取到的header用于填充`canonical_head_number`和`canonical_head`字段

```Rust
#[derive(Debug)]
struct ChainInfoInner {
    /// 收到的上一次Fork choice更新的时间戳.
    ///
    /// 主要用于追踪是否连接到一个beacon node.
    last_forkchoice_update: RwLock<Option<Instant>>,
    /// 上次交换过渡（transition）配置的时间戳.
    ///
    /// T主要用于追踪是否连接到一个beacon node.
    last_transition_configuration_exchange: RwLock<Option<Instant>>,
    /// 追踪 `canonical_head`的数量.
    canonical_head_number: AtomicU64,
    /// 链的规范头部.
    canonical_head: RwLock<SealedHeader>,
    /// beacon chain认为safe的block.
    safe_block: RwLock<Option<SealedHeader>>,
    /// 信标链认为finalized的block.
    finalized_block: RwLock<Option<SealedHeader>>,
}
```
    - 为什么要重新用读取数据库而不是前面lookup_head到的header？

```Rust
// 这里读取使用的best number读取的check_point_number
fn best_block_number(&self) -> RethResult<BlockNumber> {
    Ok(self
        .get_stage_checkpoint(StageId::Finish)
        .map(|checkpoint| checkpoint.block_number)
        .unwrap_or_default())
}
// lookup_head读取的
let head = provider.get_stage_checkpoint(StageId::Finish)?.unwrap_or_default().block_number;
```

### blob_store的默认初始化

- `InMemoryBlobStoreInner`的结构如下
    - `store: RwLock<HashMap<B256, BlobTransactionSidecar>>`
        - `BlobTransactionSidecar`包含blob的元数据如下，详见EIP4844
            - `blobs:Vec<Blob>`  ：Blob当前是一个128KB的u8数组
            - `commitments:Vec<Bytes48>`  blob的commitments
            - `proofs:Vec<Bytes48>` blob的proofs
    - `data_size:AtomicUsize`
    - `num_blobs:AtomicUsize`

## 验证工作

### 初始化一个validator

#### 初始化builder

- 根据`chain_spec`初始化一个`EthTransactionValidatorBuilder`
- 然后给`EthTransactionValidatorBuilder`添加head的时间戳；
- 添加kzg的配置：比如主网的启动raw位于crates/primitives/res/eip4844下的constant文件；
- 添加additional_tasks的数量，这里为1；

#### builder运行生成validator

- 然后build一个`TransactionValidationTaskExecutor<EthTransactionValidator<Client, Tx>>`
    - 在这里面根据相关特性的开关进行初始化一个`EthTransactionValidatorInner`，比如如下的特性：shanghai, cancun, eip2718,  eip1559,  eip4844,  block_gas_limit,  minimum_priority_fee,  additional_tasks,  propagate_local_transactions,  kzg_settings等；
    - 升级采取ForkTracker进行判断，比如上海升级或者Cancun升级是否已经激活；

> `EthTransactionValidatorInner`也是一个跟链上交易验证强相关的一个结构；

- 构建一个ValidationTask channel
    - 构建上述的特性相关的`EthTransactionValidatorInner`
    - 构建一个validationTask的channel
        - mpsc::channel

### 初始化transaction-pool

- 就是持有上述的validator和blobStore和当前的`tx_pool`的`config`
- 但是PooInner不止是上述两个结构，初始化时候采用默认实现

```Rust
impl<V, T, S> PoolInner<V, T, S>
where
    V: TransactionValidator,
    T: TransactionOrdering<Transaction = <V as TransactionValidator>::Transaction>,
    S: BlobStore,
{
    /// Create a new transaction pool instance.
    pub(crate) fn new(validator: V, ordering: T, blob_store: S, config: PoolConfig) -> Self {
        Self {
            identifiers: Default::default(),
            validator,
            event_listener: Default::default(),
            pool: RwLock::new(TxPool::new(ordering, config.clone())),
            pending_transaction_listener: Default::default(),
            transaction_listener: Default::default(),
            blob_transaction_sidecar_listener: Default::default(),
            config,
            blob_store,
            blob_store_metrics: Default::default(),
        }
    }
}

```

### spawn txpool主要的task

- 订阅CanonState的channel，还是读取的`self.tree.subscribe_to_canonical_state()`；读取的就是前面BlockchainTree的构建的`canonical`的相关的channel，这里以subscribe的形式进行接收，得到`chain_events`；
- 结合`BlockchainProvider`构建txpool maintenance task；主要任务位于transaction-pool的`src/maintain`中的maintain_transaction_pool实现，