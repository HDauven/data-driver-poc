#![no_std]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use wasm_bindgen::prelude::*;

/// Set dlmalloc as the global allocator for heap allocs
#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

/// Sends logs to the JS console
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Contract data that we will ser/deser from JSON <-> RKYV
#[derive(Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize)]
pub struct ContractData {
    recipient: String,
    amount: u64,
    withdraw_event: dusk_core::transfer::WithdrawEvent,
}

/// JSON schema definition for contract data
const SCHEMA: &str = r#"
{
    "type": "object",
    "properties": {
        "recipient": { "type": "string" },
        "amount": { "type": "integer" }
    },
    "required": ["recipient", "amount"]
}
"#;

/// Returns the JSON schema of the contract
#[wasm_bindgen]
pub fn get_schema() -> String {
    SCHEMA.to_string()
}

/// Converts JSON input into RKYV
#[wasm_bindgen]
pub fn convert_json_to_rkyv(json_input: &str) -> Result<Vec<u8>, JsValue> {
    let parsed: ContractData = serde_json::from_str(json_input)
        .map_err(|e| JsValue::from_str(&format!("Invalid JSON input: {}", e)))?;

    let serialized = rkyv::to_bytes::<_, 256>(&parsed)
        .map_err(|e| JsValue::from_str(&format!("Serialization failed: {}", e)))?;

    Ok(serialized.to_vec())
}

// Deserializes RKYV payload back into JSON
#[wasm_bindgen]
pub fn process_rkyv_payload(rkyv_payload: &[u8]) -> Result<String, JsValue> {
    let archived = unsafe { rkyv::archived_root::<ContractData>(rkyv_payload) };

    let deserialized: ContractData = archived
        .deserialize(&mut rkyv::Infallible)
        .map_err(|e| JsValue::from_str(&format!("Deserialization failed: {:?}", e)))?;

    serde_json::to_string(&deserialized)
        .map_err(|e| JsValue::from_str(&format!("Serialization to JSON failed: {}", e)))
}
