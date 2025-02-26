use rkyv::{Archive, Deserialize, Serialize};
use serde::Deserialize as SerdeDeserialize;
use serde::Serialize as SerdeSerialize;
use std::alloc::{alloc as salloc, dealloc as sdealloc, Layout};

#[derive(Archive, Serialize, Deserialize, Debug, SerdeSerialize, SerdeDeserialize)]
#[archive_attr(derive(Debug))]
pub struct ContractData {
    pub recipient: String,
    pub amount: u64,
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

#[no_mangle]
pub extern "C" fn get_schema() -> *const u8 {
    let bytes = SCHEMA.as_bytes();
    let ptr = unsafe { alloc(bytes.len()) };
    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len()) };
    ptr
}

#[no_mangle]
pub extern "C" fn get_schema_len() -> usize {
    SCHEMA.len()
}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    unsafe { salloc(Layout::from_size_align(size, 1).unwrap()) }
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, size: usize) {
    unsafe { sdealloc(ptr, Layout::from_size_align(size, 1).unwrap()) }
}

#[no_mangle]
pub extern "C" fn json_to_rkyv(ptr: *const u8, len: usize, size: *mut usize) -> *const u8 {
    let json_data = unsafe { std::slice::from_raw_parts(ptr, len) };
    let json_str = std::str::from_utf8(json_data).expect("Invalid UTF-8");

    let parsed: ContractData = serde_json::from_str(json_str).expect("Invalid JSON format");
    let serialized = rkyv::to_bytes::<_, 256>(&parsed).expect("Serialization failed");

    let output_ptr = unsafe { salloc(Layout::from_size_align(serialized.len(), 1).unwrap()) };
    unsafe { std::ptr::copy_nonoverlapping(serialized.as_ptr(), output_ptr, serialized.len()) };

    unsafe { *size = serialized.len() };
    output_ptr
}

#[no_mangle]
pub extern "C" fn rkyv_to_json(ptr: *const u8, len: usize, size: *mut usize) -> *const u8 {
    let rkyv_data = unsafe { std::slice::from_raw_parts(ptr, len) };

    let archived = unsafe { rkyv::archived_root::<ContractData>(rkyv_data) };
    let deserialized: ContractData = archived
        .deserialize(&mut rkyv::Infallible)
        .expect("Deserialization failed");

    let json_output = serde_json::to_string(&deserialized).expect("Serialization to JSON failed");

    let output_ptr = unsafe { salloc(Layout::from_size_align(json_output.len(), 1).unwrap()) };
    unsafe { std::ptr::copy_nonoverlapping(json_output.as_ptr(), output_ptr, json_output.len()) };

    unsafe { *size = json_output.len() };
    output_ptr
}
