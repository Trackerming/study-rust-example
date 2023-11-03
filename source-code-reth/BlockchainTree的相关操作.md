## 项目初始化期间的BlockchainTree的操作

### new操作

- 主要初始化操作如下
    - 构建了一个broadcast channel
    - 从数据库中读取到`last_canonical_hashes`，目测是稳定的block的`BlockNumber`和`BlockHash`；
    - 判断`last_canonical_hashes`获取的block的数量与最大reorg的深度做对比，比`max_reorg_depth`小则取最后的值作为`last_finalized_block_number`；否则就取数组中的index为`max_reorg_depth`的值作为`last_finalized_block_number`；
- BlockchainTree的主要结构就包含如下
    - `externals`：还是db_clone、chain_spec_clone、 consensus、和revm的factory
    - `state`：上述初始化操作得到的相关的block和hash，如`last_finalized_block_number`和`last_canonical_hashes`
    - `canon_state_notification_sender`：channel的sender；
    - `metrics`：初始化时候的默认实现
    - `sync_metrics_tx`：就是前面metrics channel的发送端
      - 见项目使用的channel记录中的unbounded_channel
    - `prune_modes`：主流程中按照prune的配置得到的prune_modes

### ShareableBlockchainTree

- 由于上述的tree结构上的特点，需要采用Arc和RwLock封装进行线程间安全的同步共享与修改

```Rust
#[derive(Clone, Debug)]
pub struct ShareableBlockchainTree<DB: Database, EF: ExecutorFactory> {
    /// BlockchainTree
    pub tree: Arc<RwLock<BlockchainTree<DB, EF>>>,
}
```
- 采用RwLock说明读多写少；