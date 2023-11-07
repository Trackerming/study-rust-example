# 项目使用的channel记录

### unbounded\_channel

-   来源：`NodeCommand.execute`中的
    ```rust
     let (metrics_tx, metrics_rx) = unbounded_channel();
    ```
-   channel特点
    -   api文档链接：[https://docs.rs/tokio/1.33.0/tokio/sync/mpsc/index.html](https://docs.rs/tokio/1.33.0/tokio/sync/mpsc/index.html "https://docs.rs/tokio/1.33.0/tokio/sync/mpsc/index.html")
    -   tokio中的mpsc，多生产者单消费者通道
        -   创建unbounded  mpsc 通道，用于在没有backpressure下的异步任务之间进行通信；
        -   对应`std::sync`下的`mpsc`的`unbounded_channel`
    -   特点
        -   只要接收部分尚未关闭，此通道上的 `send` 将始终成功。如果接收方落后，消息将被任意缓冲；
        -   注意：可用系统内存量是与通道的隐式绑定。使用 `unbounded` 通道可能会导致进程耗尽内存。在这种情况下，该过程将被中止；
-   业务特点
    -   通道内发送消息为MetricEvent，具体的定义如下
        ```rust
        #[derive(Clone, Copy, Debug)]
        pub enum MetricEvent {
            /// Sync reached new height. All stage checkpoints are updated.
            SyncHeight {
                /// Maximum height measured in block number that sync reached.
                height: BlockNumber,
            },
            /// Stage reached new checkpoint.
            StageCheckpoint {
                /// Stage ID.
                stage_id: StageId,
                /// Stage checkpoint.
                checkpoint: StageCheckpoint,
                /// Maximum known block number reachable by this stage.
                /// If specified, `entities_total` metric is updated.
                max_block_number: Option<BlockNumber>,
            },
            /// Execution stage processed some amount of gas.
            ExecutionStageGas {
                /// Gas processed.
                gas: u64,
            },
        }
        ```
    -   接收端由metrics\_listener封装，并采用`ctx.task_executor.spawn_critical`启动监听metrics的任务，采用一系列将future封装成task，最后spawn到runtime上；
    -   发送端有如下几个地方
        -   构建`blockchain_tree`的时候的`with_sync_metrics_tx`
        -   配置`pipeline`的时候的`build_networked_pipeline`

### braodcast::channel

- 来源

    - `BlockchainTree`初始化中的代码如下

    ```Rust
    let (canon_state_notification_sender, _receiver) =
    tokio::sync::broadcast::channel(max_reorg_depth as usize * 2);
    ```

    > 上述代码可知没有当场使用receiver，后续应该是采用sender的subscribe创建新的receiver

- channel特点
    - api文档链接：[https://docs.rs/tokio/1.33.0/tokio/sync/broadcast/index.html](https://docs.rs/tokio/1.33.0/tokio/sync/broadcast/index.html)，概况可见：[https://tokio.rs/tokio/tutorial/channels](https://tokio.rs/tokio/tutorial/channels)
    - tokio中的mpmc广播channel
    - 特点
        - 多生产者、多消费者广播队列。每个发送的值都会被所有消费者看到；
        - `Sender` 用于将值广播到所有连接的 `Receiver` 值。 `Sender` 句柄是可克隆的，允许并发发送和接收操作。 `Sender` 和 `Receiver` 都是 `Send` 和 `Sync` ，只要 `T` 是 `Send`
        - 当发送值时，所有 `Receiver` 句柄都会收到通知并接收该值。该值在通道内存储一次，并根据每个接收器的需要进行克隆。一旦所有接收者都收到了该值的克隆，该值就会从通道中释放；
        - 通过调用 `channel` 创建通道，指定通道在任何给定时间可以保留的最大消息数
        - 新的 `Receiver` 句柄是通过调用 `Sender::subscribe` 创建的。返回的 `Receiver` 将接收调用 `subscribe` 后发送的值
        - 此通道还适用于单生产者多消费者用例，其中单个发送者向多个接收者广播值；
- 业务特点
    - canonical state是代表的什么
    - 跟reorg的深度有关，比如当前最大的reorg的深度为64，则channel的最大size就是64*2=128；注释的解释是因为最大的reorg的深度至少是N个block必须被一次发送；
  构建了一个broadcast channel

### mpsc::channel

- 来源：`crates/transaction-pool/src/validate/task`中的`ValidationTask::new`

```Rust
    pub fn new() -> (ValidationJobSender, Self) {
        let (tx, rx) = mpsc::channel(1);
        (ValidationJobSender { tx }, Self::with_receiver(rx))
    }
#[derive(Debug)]
pub struct ValidationJobSender {
    tx: mpsc::Sender<Pin<Box<dyn Future<Output = ()> + Send>>>,
}
#[derive(Clone)]
pub struct ValidationTask {
    #[allow(clippy::type_complexity)]
    validation_jobs: Arc<Mutex<ReceiverStream<Pin<Box<dyn Future<Output = ()> + Send>>>>>,
}

```
- channel特点
    - **多生产者**功能允许从多个任务发送消息。创建通道会返回两个值：发送者和接收者。两个手柄分开使用。他们可能会被转移到不同的任务；**单消费**者，这里就是指的创建这个channel的`ValidationTask`本身，且不可被复制；
    - 创建的通道容量为 32。如果消息发送速度快于接收速度，通道将存储它们。一旦 32 条消息存储在通道中，调用 `send(...).await` 将进入休眠状态，直到接收者删除一条消息；
    - 当每个 `Sender` 超出范围或已被删除时，就不再可能向通道发送更多消息。此时，对 `Receiver` 的 `recv` 调用将返回 `None` ，这意味着所有发送者都消失了，通道已关闭；
- 业务特点
    - 就是`ValidationTask`生成了一个mpsc的channel，将发送端的句柄扔出来，保留接收端；
    - 然后`tasks.spawn_blocking`启动一个任务用于`ValidationTask`接收task并执行其run方法；注释的因为它们执行db的loopup致使blocking啥意思？
    - 随后又采用`tasks.spawn_critical_blocking`显示命名transaction-validation-service运行task的run？没看懂这里不是单接收者么，运行两个spawn处理不还是一个一个的来？
    - 然后将发送端的句柄封装成Arc<Mutex<>>结构传递出去；