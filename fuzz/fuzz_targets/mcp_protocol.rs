#![no_main]

use libfuzzer_sys::fuzz_target;
use serde_json::Value;

fuzz_target!(|data: &[u8]| {
    // Fuzz MCP JSON-RPC protocol parsing
    if let Ok(s) = std::str::from_utf8(data) {
        // Try to parse as JSON
        let _ = serde_json::from_str::<Value>(s);
        
        // Try to parse as MCP request
        // This tests the robustness of JSON parsing without panicking
    }
});
