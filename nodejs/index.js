const express = require('express')
const https = require('https')
const fs = require('fs')
const app = express()

const options = {
  key: fs.readFileSync("../certs/key.key"),
  cert: fs.readFileSync("../certs/cert.crt")
};

app.use(express.json())
const port = 3003

app.get('/json', (req, res) => {
  res.json({ value: "value" });
})

app.get('/text', (req, res) => {
  res.send("value");
})

app.post('/body', (req, res) => {
  res.json(req.body);
})


if (process.env.USE_TLS) {
  console.log(`Node app listening at https://localhost:${port}`)
  https.createServer(options, app).listen(port);
} else {
  app.listen(port, () => {
    console.log(`Node app listening at http://localhost:${port}`)
  });
}