use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

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

