import http from 'node:http';
import {RequestOptions} from "http";

(async function (port: number, host: string) {
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
        req.write(JSON.stringify(body));
        req.end();
    })
})(8443, '127.0.0.1')
