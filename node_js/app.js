const express = require('express');
const cassandra = require('cassandra-driver')
const app = express();

let client = new cassandra.Client({
    contactPoints: [
        "127.0.0.1:9001",
        "127.0.0.1:9002",
        "127.0.0.1:9003"
    ],
    localDataCenter: 'datacenter1',
    keyspace: 'test'
});


app.get('/patents/:serialNumber', async (req, res) => {
    try {
        let result = await client.execute("SELECT * FROM patent WHERE serial_number = ? ", [req.params.serialNumber], { prepare: true });
        res.json(result.rows[0]);
    } catch (err) {
        res.send(err);
    }
});



app.listen(5000);