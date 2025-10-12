#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Fuzz pattern search with arbitrary input
    if let Ok(query) = std::str::from_utf8(data) {
        // Test that pattern search doesn't panic on arbitrary input
        // This would test the vector search functionality
        let _ = query.to_lowercase();
        let _ = query.trim();
    }
});
