#! /bin/sh
/app/socket_relay_bin --source-address "127.0.0.1:9443" --destination-address "18:8080" &
ts-node /app/server.ts
