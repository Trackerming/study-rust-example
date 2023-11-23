#! /bin/sh
/app/socket_relay_bin --tcp-address "127.0.0.1:9443" --vsock-address "16:8888"  &
ts-node /app/server.ts
