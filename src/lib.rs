mod utils;

use wasm_bindgen::prelude::*;
use openapi::models::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn xform(json: &str) -> String  {
    let mut deployment: IoK8sApiAppsV1Deployment = serde_json::from_str(json).unwrap();
    deployment.api_version = match deployment.api_version {
        Some(value) => Some(value),
        None => Some("apps/v1".to_string()),
    };
    return serde_json::to_string(&deployment).unwrap();
}