### 初始化配置

#### 加载网络配置

#### secret key

- 作用：唯一标识节点，这里有ENode的概念
- 初始化过程
    - 读取secret key通过`NetworkArgs`的`p2p_secret_ke`路径配置；默认在数据目录的`discovery-secret`;
    - 得到secret key是32字节的u8数组；

#### peers path

- 数据目录下的`known-peers.json`文件

#### 构建NetworkConfigBuilder

- 基础配置相关
    - 传入上述的sec key、peers的配置，和config、ChainSpec
    - 从chainspec中加载bootnodes，没匹配到则默认为主网的nodes
    - peers的相关配置从config.peers中获取，比如传入传出节点的上限数量
    - 配置基础的网络相关的配置比如nat、peers的持久化存储（就是上面的konwn-peers.json文件）和sec key和前面的配置构建成`NetworkConfigBuilder`
    - `pk2id(&self.secret_key.public_key(SECP256K1))`获取节点的id，即peerId
    - 构建`hello_message`字段，HelloMessage涵盖如下信息
        - protocol_version：P2P协议版本，比如V5
        - client_version：软件运行版本，比如"reth/v0.1.0-alpha.10-d3de32a9/x86_64-apple-darwin"
        - capabilities：eth66 eth67 eth68
        - port：默认30303
        - id(peer_id)
    - 应用discv，根据配置进行dns和discv4 discovery的配置；dev mode下都为None
- 依然传入了`task_executor`
- setHead作用是，设置自己节点的head便于与P2P节点进行交换信息？
- listen在0.0.0.0:30303端口上，会区分instance数量，比如程序的启动端口为30303+instance-1；避免端口冲突；discv_addr的地址同一个；
- 传入`ProviderFactory`，

##### 基于builder构建NetworkConfig

- 主要的配置builder设置清楚了，head数据库等也准备就绪

##### 基于config启动network