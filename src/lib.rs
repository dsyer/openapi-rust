#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

mod models;

#[wasm_bindgen(module = "runtime")]
extern "C" {
    async fn getWithHeaders(url: &str, headers_json: &str) -> JsValue;
}

#[wasm_bindgen]
pub async fn xform() -> JsValue {
    unsafe {
        return getWithHeaders("http://localhost:5000/v2/", "{}").await;
    }
}
