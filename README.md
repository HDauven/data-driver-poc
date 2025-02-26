# Data Driver Proof of concept

This project demonstrates how to use Rust and WASM to serialize JSON data into RKYV and deserialize it back into JSON. The goal is to provide an efficient serialization format for contracts while maintaining interoperability with non-Rust clients.

## Compiling to WASM

To compile the Rust code into a WASM package:
```sh
wasm-pack build --target nodejs --release
```

The WASM binary and JS entrypoint can be found at:
```
./pkg/data_driver_demo_bg.wasm
./pkg/data_driver_demo.js
```

For other targets, see: [wasm-pack targets](https://rustwasm.github.io/wasm-pack/book/commands/build.html#target)

## Running the JavaScript client

A test client script is available to interact with the compiled WASM package, converting JSON to RKYV and back.

### Install dependencies

```sh
npm install
```

### Run client

Make sure the WASM package is compiled and available in the right directory. To run the client:
```
npm run execute
```

If everything is working correctly, you should get the following result:
```js
Schema: {
  type: 'object',
  properties: { recipient: { type: 'string' }, amount: { type: 'integer' } },
  required: [ 'recipient', 'amount' ]
}
JSON Input: { recipient: '0x12345', amount: 1000 }
RKYV Payload (Hex): 3078313233343507e803000000000000
Deserialized JSON: { recipient: '0x12345', amount: 1000 }
```
