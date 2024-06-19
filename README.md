# study-rust-example
> 日常学习和技术研究示例，包括从0开始的Rust以及crypto currency相关的一些基础知识以及常见工具cli的实现等等；

### 结构
- 介绍
  - workspace的模块结构，每个类目新增一个member对应到根目录
- 详细
  - algorithms 常见算法的实现
    - algorithms-utils 实现常见算法中比较常见的方法
  - async-web-server 异步web服务的学习示例
  - **crypto** 加密相关知识的模块学习与示例
    - crypto_small_math 在较小的计算范围内的一些基础原理演示
    - crypto_util 常见的一些方法拆分成独立的模块供其他模块调用
    - des des的示例
    - ecc_simple_demo ECC相关算法的实现以及示例
    - enc_dec
      - 加密上常见的一些基础编解码如Base58、bech32、rlp等
    - jwt_demo jwt的功能演示
    - res
      - 一些基础过程的一些文档图资源
    - rsa_simple_demo RSA的算法的基础原理与实现演示
    - utxo_wallet utxo的一些交易的示例
    - zero_knowledge 零知识证明的相关记录与演示实现
  - games 依赖bevy实现的简单小的游戏示例
  - hello_macro_derive 简单宏示例
  - list 圣经中对list的实现的一些示例过程
  - minigrep 小的grep cli示例命令行程序
  - redis-simple 简单的redis的操作示例
  - socket-relay vsock/Tcp转换工具
  - **source-code-reth** reth源码分析记录
  - study-example Rust基础知识的学习示例
  - study-practice Rust中进阶知识的一些练习程序
  - **tool** 加密货币中常见的一些功能实现的命令行工具，BTC/ETH的一些基础功能
  - vsock-node 一个aws上使用TEE的一些示例程序
  - web-server 简单同步的一些web服务示例

> 大量基础练习的例子来自于官方的推荐学习文档 [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/) 示例代码书写
