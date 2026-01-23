const http = require('http');
const fs = require('fs');
const { exec } = require('child_process');
http.createServer((req, res) => {
    res.setHeader('Access-Control-Allow-Origin', '*');
    if (req.url === '/view') {
        res.writeHead(200, {'Content-Type': 'text/html'});
        fs.createReadStream('dashboard.html').pipe(res);
    } else {
        exec('./cpu_mon', (err, stdout) => {
            res.writeHead(200, {'Content-Type': 'application/json'});
            res.end(stdout || '{"cpu_load":0.0}');
        });
    }
}).listen(3000, '0.0.0.0', () => console.log('>> [NEXUS] BRIDGE LIVE'));
