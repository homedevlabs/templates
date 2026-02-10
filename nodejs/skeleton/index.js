const express = require('express');
const app = express();
const port = 80;

app.get('/', (req, res) => {
    res.send('<h1>Hello from Node.js (Backstage Template)</h1><p>Managed by Traefik at superlab.app</p>');
});

app.listen(port, () => {
    console.log(`App listening at http://localhost:${port}`);
});
