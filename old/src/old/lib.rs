use wasm_bindgen::prelude::*;

mod utils;

use utils::*;

#[wasm_bindgen(start)]
pub fn rust_init() {
    console_log!("Initializing in Rust...");

    // Init code
    set_panic_hook();

    console_log!("Rust successfully initialized.")
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello! {}!", name));
}
