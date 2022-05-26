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
pub async fn xform(value: String) -> JsValue {
    unsafe {
        return getWithHeaders(
            &value,
            r#"{"accept": "application/vnd.docker.distribution.manifest.v2+json"}"#,
        )
        .await;
    }
}

