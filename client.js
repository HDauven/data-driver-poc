import fs from 'fs';

async function main() {
    const wasmBuffer = await fs.readFileSync('./target/wasm32-unknown-unknown/release/data_driver_demo.wasm');
    const wasmModule = await WebAssembly.instantiate(wasmBuffer);
    const { instance } = wasmModule;
    const { alloc, dealloc, get_schema, get_schema_len, json_to_rkyv, rkyv_to_json, memory } = instance.exports;

    // Fetch the schema
    const schemaLen = get_schema_len();
    const schemaPtr = get_schema();

    const wasmMemory = new Uint8Array(memory.buffer);
    const schemaBytes = wasmMemory.slice(schemaPtr, schemaPtr + schemaLen);
    const schema = new TextDecoder().decode(schemaBytes);

    console.log("Schema:", JSON.parse(schema)); // Parse and log the schema

    // Helper function to write JSON into WASM memory
    function writeToWasmMemory(str) {
        const encoder = new TextEncoder();
        const bytes = encoder.encode(str);

        // Allocate memory in the WASM module
        const ptr = alloc(bytes.length);

        // Write the encoded bytes into WASM memory
        const wasmMemory = new Uint8Array(memory.buffer, ptr, bytes.length);
        wasmMemory.set(bytes);

        return { ptr, len: bytes.length };
    }

    // Create JSON input based on the schema
    const jsonInput = {
        recipient: "0x12345",
        amount: 1000,
    };

    console.log("JSON Input:", jsonInput);

    // Write JSON into WASM memory
    const { ptr, len } = writeToWasmMemory(JSON.stringify(jsonInput));

    // Allocate space for the size of the RKYV payload
    const sizePtr = alloc(4);

    // Call the WASM function to convert JSON to RKYV
    const rkyvPtr = json_to_rkyv(ptr, len, sizePtr);

    // Read the size of the RKYV payload
    const sizeBytes = new Uint32Array(memory.buffer, sizePtr, 1);
    const rkyvSize = sizeBytes[0];

    // Read the RKYV payload from WASM memory
    const rkyvPayload = wasmMemory.slice(rkyvPtr, rkyvPtr + rkyvSize);
    console.log("RKYV Payload (Hex):", Buffer.from(rkyvPayload).toString('hex'));

    // Deserialize the RKYV payload back to JSON
    const deserializedSizePtr = alloc(4);
    const jsonOutputPtr = rkyv_to_json(rkyvPtr, rkyvSize, deserializedSizePtr);

    // Read the size of the deserialized JSON
    const jsonOutputSize = new Uint32Array(memory.buffer, deserializedSizePtr, 1)[0];

    // Read the deserialized JSON from WASM memory
    const jsonOutputBytes = wasmMemory.slice(jsonOutputPtr, jsonOutputPtr + jsonOutputSize);
    const jsonOutput = new TextDecoder().decode(jsonOutputBytes);
    console.log("Deserialized JSON:", JSON.parse(jsonOutput));

    // Cleanup WASM memory
    dealloc(ptr, len);
    dealloc(rkyvPtr, rkyvSize);
    dealloc(sizePtr, 4);
    dealloc(jsonOutputPtr, jsonOutputSize);
    dealloc(deserializedSizePtr, 4);
}

main().catch(console.error);
