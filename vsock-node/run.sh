#! /bin/sh
/app/vsock-node vsock_to_tcp_server --host "127.0.0.1" --tcpPort 8888 --port 8686
#ts-node /app/enclave.ts
