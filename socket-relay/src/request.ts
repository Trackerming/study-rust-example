import http from 'node:http';
import {RequestOptions} from "http";

async function request(options: RequestOptions, body: string) {
    return new Promise<string>((resolve, reject) => {
        const req = http.request(options, (res) => {
            let data = '';
            res.on('data', (chunk) => {
                data += chunk;
            });
            res.on('end', () => {
                console.log(`Response from server: ${data}`);
                return resolve(data);
            });
        });
        req.on('error', (error) => {
            const errMsg = `Error making request: ${error.message}`;
            console.error(errMsg);
            reject(errMsg);
        });
        req.write(body);
        req.end();
    })
}

(async function (port: number, host: string, loopTimes: number) {
    const body = {a: 100, b: "100"};
    let options: RequestOptions = {
        hostname: host,
        port: port,
        path: '/path',
        method: 'POST',
    };
    options.headers = {
        'Content-Type': 'application/json',
        'Content-Length': Buffer.byteLength(JSON.stringify(body)),
    };
    for (let i = 0; i < loopTimes; i++) {
        if (i % 2 == 0) {
            options.path = '/path'
        } else {
            options.path = '/path1'
        }
        await request(options, JSON.stringify(body));
    }
})(8443, '127.0.0.1', 1024)
