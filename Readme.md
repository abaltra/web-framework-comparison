# Performance comparison of different web frameworks.

Currently listed ones:
* Atreugo (Go)
* .NET Core webapi (C#)
* Actix (Rust)
* Express (Javascript)

Each one of these services supports 3 endpoints:
* `GET /text`, which returns `"value"` as a `plaintext` response
* `GET /json`, which returns `{ "value": "value" }` as an `application/json` response
* `GET /body`, which receives a generic `JSON` payload and returns it verbatim as `application/json`

All were tested using [loadtest](https://www.npmjs.com/package/loadtest), with 10000 requests and a concurrency of 100. The results of each run can be found in the `benchmarks` folder for each framework.