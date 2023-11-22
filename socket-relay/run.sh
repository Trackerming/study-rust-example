#! /bin/sh
/app/socket_relay_bin --tcp-address "127.0.0.1:9443" --vosck-address "16:8080"  &
ts-node /app/server.ts
