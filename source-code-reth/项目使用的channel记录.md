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
