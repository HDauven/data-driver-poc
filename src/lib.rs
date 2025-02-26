use rkyv::{Archive, Deserialize, Serialize};
use serde::{Serialize as SerdeSerialize, Deserialize as SerdeDeserialize};
use wasm_bindgen::prelude::*;

#[derive(Archive, Serialize, Deserialize, Debug, SerdeSerialize, SerdeDeserialize)]
pub struct ContractData {
    recipient: String,
    amount: u64,
}

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

#[wasm_bindgen]
pub fn get_schema() -> String {
    SCHEMA.into()
}

#[wasm_bindgen]
pub fn convert_json_to_rkyv(json_input: &str) -> Result<Vec<u8>, JsValue> {
    let parsed: ContractData = serde_json::from_str(json_input)
        .map_err(|e| JsValue::from_str(&format!("Invalid JSON input: {}", e)))?;

    let serialized = rkyv::to_bytes::<_, 256>(&parsed)
        .map_err(|e| JsValue::from_str(&format!("Serialization failed: {}", e)))?;

    Ok(serialized.to_vec())
}

#[wasm_bindgen]
pub fn process_rkyv_payload(rkyv_payload: &[u8]) -> Result<String, JsValue> {
    let archived = unsafe { rkyv::archived_root::<ContractData>(rkyv_payload) };

    let deserialized: ContractData = archived
        .deserialize(&mut rkyv::Infallible)
        .map_err(|e| JsValue::from_str(&format!("Deserialization failed: {:?}", e)))?;

    serde_json::to_string(&deserialized)
        .map_err(|e| JsValue::from_str(&format!("Serialization to JSON failed: {}", e)))
}
