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