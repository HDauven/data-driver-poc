const wasm = require('./pkg/data_driver_demo.js');

async function main() {
    // Fetch and display the schema
    console.log("Schema:", JSON.parse(wasm.get_schema()));

    // Example JSON input
    const jsonInput = JSON.stringify({
        recipient: "0x12345",
        amount: 1000,
    });

    console.log("JSON Input:", jsonInput);

    try {
        // Convert JSON to RKYV
        const rkyvPayload = wasm.convert_json_to_rkyv(jsonInput);
        console.log("RKYV Payload (Hex):", Buffer.from(rkyvPayload).toString('hex'));

        // Process the RKYV payload and convert back to JSON
        const jsonOutput = wasm.process_rkyv_payload(rkyvPayload);
        console.log("Deserialized JSON:", JSON.parse(jsonOutput));
    } catch (error) {
        console.error("Error:", error);
    }
}

main().catch(console.error);
