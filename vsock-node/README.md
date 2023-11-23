#### 功能拆解

##### vsock通信部分

> 在实例上接收proxy的请求，即ec2实例上需要启动一个vsock的server

- 数据发送
    - 以vsock的方式发送数据
- 数据接收
    - 接收http的请求数据
    - 发送通过vsock的方式

##### http通信部分

> enclave内部需要启动一个proxy，用于接收vsock的数据并转发为http的数据给应用程序

- 数据发送
    - 发送通过http的数据进行还原
- 数据接收
    - 接收vsock的请求数据
    - 发送通过http的方式

```shell
docker build -t testenclave:v1 .
sudo nitro-cli build-enclave --docker-uri testenclave:v1 --output-file testenclave.eif
sudo nitro-cli run-enclave --eif-path testenclave.eif --memory 4096 --cpu-count 2  --enclave-cid 16 --debug-mode
nitro-cli console --enclave-id i-0d45180d8ac4256cb-enc18b41ce18e55b8d
./vsock-node tcp_to_vsock_server --cid 17 --port 8686 --tcpPort 9332 --host "127.0.0.1"
./vsock-node tcp_client --tcpPort 9332 --host "127.0.0.1"
sudo nitro-cli terminate-enclave --enclave-name testenclave
```