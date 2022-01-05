use wasm_bindgen::prelude::*;

pub mod set1;
pub mod text_utils;

#[wasm_bindgen(start)]
pub fn wasm_main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    Ok(())
}
