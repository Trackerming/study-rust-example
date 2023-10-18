import net from 'node:net';

let server = net.createServer((socket) => {
  socket.on('data', function (data) {
    console.log(data);
    let resp = 'hi this is enclave response';
    socket.write(resp, () => {
      console.log(`server resp: ${resp}`);
    });
  });
});

server.listen(8888, () => {
  console.log('Tcp server start');
});
