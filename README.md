# Data Driver Proof of concept

This project demonstrates how to use Rust and WASM to serialize JSON data into RKYV and deserialize it back into JSON. The goal is to provide an efficient serialization format for contracts while maintaining interoperability with non-Rust clients.

## Compiling the WASM module

To compile the Rust code into a WASM module:
```sh
cargo build --target wasm32-unknown-unknown --release
```

The WASM binary can be found at:
```
target/wasm32-unknown-unknown/release/data_driver_demo.wasm
```

## Running the JavaScript client

For testing purposes there's a client script available that interacts with the compiled WASM module, converting JSON to RKYV and back.

### Install dependencies

```sh
npm install
```

### Run client

Make sure the WASM module is compiled and available in the right directory. To run the client:
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
