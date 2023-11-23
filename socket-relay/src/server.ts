import http from 'http';

const server = http.createServer((req, res) => {
    let data = '';
    req.setEncoding('utf8');
    req.on('data', (chunk) => {
        data += chunk;
    });
    req.on('end', () => {
        console.log(req.headers);
        console.log(req.method);
        console.log(req.url);
        console.log(`Received data: ${data}`);
        res.writeHead(200, {'Content-Type': 'text/plain'});
        res.end('Hello, World!\n' + data);
    })
});

const port = 9443;
const host = "127.0.0.1";
server.listen(port, host, () => {
    console.log(`Server listening on address ${JSON.stringify(server.address())}`);
});
