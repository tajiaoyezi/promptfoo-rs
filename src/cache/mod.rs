pub mod resume;

use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CacheKeyInput {
    pub provider_id: String,
    pub provider_config: Value,
    pub prompt: String,
    pub test_case: TestCaseKeyInput,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TestCaseKeyInput {
    pub vars: BTreeMap<String, String>,
    pub assertions: Vec<Value>,
}

pub fn cache_key(input: &CacheKeyInput) -> String {
    let canonical =
        serde_json::to_vec(input).expect("cache key input should always serialize to JSON");
    let digest = Sha256::digest(canonical);
    format!("sha256:{}", to_hex(&digest))
}

fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}
