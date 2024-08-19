use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! This message is served to you by compiling Rust code into WebAssembly (.wasm) and JavaScript (.js)", name)
}

