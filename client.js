const wasm = require('./pkg/data_driver_demo.js');

async function main() {
    // Fetch and display the schema
    console.log("Schema:", JSON.parse(wasm.get_schema()));

    // Example JSON input
    const jsonInput = JSON.stringify({
        "recipient": "0x12345",
        "amount": 1000,
        withdraw_event : {
            "sender": "a22427226377cc867d51ad3f130af08ad13451de7160efa2b23076fd782de967",
            "receiver": {
                "Moonlight": "24zKBvRuJK3mjT8o3p4zuh1c1cHLRcSQ81QQgxCUozw3U1HukJWWX51R37fMi4C6Qac7hVoruG56fDyqdkmeopdpKowQ9APTpS94vgoZge8X6Bpuqm1dpy8wHaaRSZz1z5AN"
            },
            "value": "10644393278569127094"
        }
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
