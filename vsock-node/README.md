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
