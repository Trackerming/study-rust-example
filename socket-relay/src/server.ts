//import http from 'http';
import express from 'express';
import bodyParser from "body-parser";

/*const server = http.createServer((req, res) => {
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
});*/

const port = 9443;
const host = "127.0.0.1";
/*server.listen(port, host, () => {
    console.log(`Server listening on address ${JSON.stringify(server.address())}`);
});*/

const app = express();
app.use(bodyParser.json());
let num = 0;
app.post('/path', (req, res) => {
    console.log(JSON.stringify(req.body));
    num += 1;
    res.send(`res num from /path: ${num}, body: ${JSON.stringify(req.body)}`);
})

app.post('/path1', (req, res) => {
    console.log(JSON.stringify(req.body));
    num += 1;
    res.send(`res num from /path1: ${num}, body: ${JSON.stringify(req.body)}`);
})

app.listen(port, host);
console.log(`listen on ${port}`);
