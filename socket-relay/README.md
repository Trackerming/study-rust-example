#### socket-relay

##### 设计思路与原理

- 参考socat工具的功能，具体关于socat的使用详见MPC-DOC中的vsock的ts的验证中；
- 即在enclave和enclave所在父实例上添加一层代理；将外部的TCP请求转换为vsock的socket，通过vsock通信到enclave 内部转换为TCP的实现；
  - 所以大致的流程是tcp转vsock，vsock转tcp到达enclave的内部，然后响应数据，socat的过程如下

      - enclave内部的代理如下

    ```shell
    # 代理，将传入的vsock连接转发到本地8888端口
    socat VSOCK-LISTEN:8001,fork,reuseaddr TCP:127.0.0.1:8888 &
    # 将本地的tcp请求转发连接到父实例（vsock为3）上的8002端口
    socat TCP-LISTEN:443,fork,reuseaddr VSOCK-CONNECT:3:8002
    ```

- 父亲实例上的代理如下

  ```shell
  ENCLAVE_CID=$(nitro-cli describe-enclaves | jq -r ".[0].EnclaveCID")
  socat TCP-LISTEN:8001,fork,reuseaddr VSOCK-CONNECT:${ENCLAVE_CID}:8001
  ```

- 原理如下
    - 父实例上启动一个proxy，用于转换tcp stream为vsock的stream
    - enclave内部启动一个proxy，用于转换vsock stream到tcp stream
  > 因为目前的设计是enclave的程序都是处理外部的请求，没有主动请求到外部的数据，相比于socat，因为固定了stream的信道，所以不需要转换两次，转换信道上的数据会转发到原始协议的stream上；

- 代码设计思路
    - 由于vsock测试的不便，所以在代码层面设置一个mock-vsock的feature，对功能的接口上与vsock一致，在本地开发测试的时候可以开启mock-vsock的feature，这样转换的过程还是tcp到tcp的协议；
    -
  由于enclave内部的proxy和ec2实例上的stream转换方向不同，所以参数有tcp-to-vsock的设置，显示设置说明监听在tcp上，转换为vsock；反正则对应enclave中的应用，监听在vsock上，转换为tcp；

##### 使用方法

- 编译代码，本地编译需要安装vsock crate的依赖qemu-system-x86_64
- 显示设置Cargo.toml，mock-vsock的feature，如果开启则编译出的二进制执行文件采用的还是tcp的方式模拟vsock的接口，适用与本地开发和测试，生产实际要关闭这个feature
- cargo build获取执行文件，编译输出目录在target下；

##### 验证过程

- 本地验证过程

```shell
../target/debug/socket_relay_bin --tcp-address "127.0.0.1:9443" --vsock-address "127.0.0.1:8443"
../target/debug/socket_relay_bin --tcp-address "127.0.0.1:7443" --vsock-address "127.0.0.1:9443" --tcp-to-vsock
ts-node src/server.ts # listen 9443
ts-node src/request.ts # 请求到7443
```

- 实际验证过程

```shell
docker build -t testenclave:v1 .
sudo nitro-cli build-enclave --docker-uri testenclave:v1 --output-file testenclave.eif
sudo nitro-cli run-enclave --eif-path testenclave.eif --memory 4096 --cpu-count 2  --enclave-cid 16 --debug-mode
nitro-cli console --enclave-id i-*-enc*
./socket_relay_bin --tcp-address "127.0.0.1:8443" --vsock-address "16:8888" --max-concurrent-connections 1024 --tcp-to-vsock
ts-node src/request.ts # 请求到8443
```

##### 开发问题记录

- 启动enclave之后一直显示正常listen在vsock的端口上， 但是请求来临的时候去connect
  enclave的tcp端口的时候一直输出`network is unreachable (os error 101)`?
    - 需要Assign an IP address to local loopback，设置如下

    ```shell
    sudo ip addr add 127.0.0.1/32 dev lo
    sudo ip link set dev lo up
    ```