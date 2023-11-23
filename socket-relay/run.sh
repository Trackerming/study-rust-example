#! /bin/sh
# Assign an IP address to local loopback
sudo ip addr add 127.0.0.1/32 dev lo
sudo ip link set dev lo up
/app/socket_relay_bin --tcp-address "127.0.0.1:9443" --vsock-address "16:8888"  &
ts-node /app/server.ts
