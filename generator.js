const fs = require("fs");

function generate(method, route, body) {
    let reqTemplate;
    if (method === 'GET') {
        reqTemplate = `GET ${route} HTTP/1.1\r\nHost: hostname.com\r\nUser-Agent: tank\r\nAccept: */*\r\nConnection: Close\r\n\r\n`
    } else {
        reqTemplate = `POST ${route} HTTP/1.1\r\ncontent-type: application/json\r\nContent-Length: ${body.length}\r\nHost: xxxxxxxxx.dev.example.com\r\nUser-Agent: xxx (shell 1)\r\n\r\n${body}`
    }
    let ammoTemplate = (
        `${reqTemplate.length}\r\n${reqTemplate}`
    )

    return ammoTemplate
}

let ammo = '';

for (let i = 0; i < 200000; i++) {
    ammo += generate('POST', '/patents', JSON.stringify({ "serialNumber": i.toString(), "registrationDate": 123, "expireDate": 123, "company": "123", "img": "123", "info": "123" })) + '\r\n\r\n';
    for (let j = 0; j < 5; j++) {
        ammo += generate('GET', `/patents/${i}`) + '\r\n\r\n';
    }
}

fs.writeFileSync('ammo.txt', ammo)