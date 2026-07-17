# sydar WASM SDK

An integration wrapper around [`sydar-wasm`](https://www.npmjs.com/package/sydar-wasm) module that uses [`websocket`](https://www.npmjs.com/package/websocket) W3C adaptor for WebSocket communication.

This is a Node.js module that provides bindings to the sydar WASM SDK strictly for use in the Node.js environment. The web browser version of the SDK is available as part of official SDK releases at [https://github.com/sydarnet/sydar/releases](https://github.com/sydarnet/sydar/releases)

## Usage

sydar NPM module exports include all WASM32 bindings.
```javascript
const sydar = require('sydar');
console.log(sydar.version());
```

## Documentation

Documentation is available at [https://sydar.aspectron.org/docs/](https://sydar.aspectron.org/docs/)


## Building from source & Examples

SDK examples as well as information on building the project from source can be found at [https://github.com/sydarnet/sydar/tree/master/wasm](https://github.com/sydarnet/sydar/tree/master/wasm)

## Releases

Official releases as well as releases for Web Browsers are available at [https://github.com/sydarnet/sydar/releases](https://github.com/sydarnet/sydar/releases).

Nightly / developer builds are available at: [https://aspectron.org/en/projects/sydar-wasm.html](https://aspectron.org/en/projects/sydar-wasm.html)

