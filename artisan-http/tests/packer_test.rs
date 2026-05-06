use artisan_http::packer::Packer;
use artisan_http::packers::JsonPacker;
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_json_packer_pack() {
    let packer = JsonPacker;
    let mut data = HashMap::new();
    data.insert("key".to_string(), json!("value"));

    let result = packer.pack(&data).unwrap();
    assert_eq!(result, r#"{"key":"value"}"#);
}

#[test]
fn test_json_packer_pack_empty() {
    let packer = JsonPacker;
    let data = HashMap::new();

    let result = packer.pack(&data).unwrap();
    assert_eq!(result, "{}");
}

#[test]
fn test_json_packer_unpack() {
    let packer = JsonPacker;
    let json = r#"{"key":"value"}"#;

    let result = packer.unpack(json).unwrap();
    assert_eq!(result["key"], json!("value"));
}

#[test]
fn test_json_packer_unpack_invalid() {
    let packer = JsonPacker;
    let invalid_json = "not json";

    let result = packer.unpack(invalid_json);
    assert!(result.is_err());
}
