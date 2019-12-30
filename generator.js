const fs = require("fs");

function generate(method, route, body) {
    let reqTemplate;
    if (method === 'GET') {
        reqTemplate = `GET ${route} HTTP/1.0\r\nHost: xxx.tanks.example.com\r\nUser-Agent: xxx (shell 1)`
    } else {
        reqTemplate = `POST ${route} HTTP/1.0\r\ncontent-type: application/json\r\nContent-Length: ${body.length}\r\nHost: xxxxxxxxx.dev.example.com\r\nUser-Agent: xxx (shell 1)\r\n\r\n${body}`
    }
    let ammoTemplate = (
        `${reqTemplate.length}\r\n${reqTemplate}`
    )

    return ammoTemplate
}

let ammo = '';

for (let i = 0; i < 1; i++) {
    ammo += generate('POST', '/patents', JSON.stringify({ "serialNumber": i.toString(), "registrationDate": 123, "expireDate": 123, "company": "123", "img": "123", "info": "123" })) + '\r\n\r\n';
    for (let j = 0; j < 1; j++) {
        ammo += generate('GET', `/patents/${i}`) + '\r\n\r\n';
    }
}

fs.writeFileSync('ammo.txt', ammo)