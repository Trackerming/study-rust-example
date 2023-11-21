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
server.listen(port, () => {
    console.log(`Server listening on port ${port}`);
});
