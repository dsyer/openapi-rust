#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use std::format;

mod models;

use models::*;

#[wasm_bindgen(module = "runtime")]
extern "C" {
    async fn getWithHeaders(url: &str, headers_json: &str) -> JsValue;
    fn log(value: &str);
}

#[wasm_bindgen]
pub async fn xform(json: JsValue) -> JsValue {
    let image: V1Image = json.into_serde().unwrap();
    let mut status = V1ImageStatus::new();
    status.complete = Some(false);
    match image.spec {
        Some(spec) => {
            match spec.image {
                Some(path) => {
                    let response = fetch_image_sha(path.clone()).await;
                    match response {
                        Some(sha) => {
                            status.latest_image = Some(path + "@" + &sha);
                        },
                        None => {}
                    }                
                },
                None => {}
            }
        },
        None => {}
    }
    status.complete = if let Some(_) = status.latest_image {
        Some(true)
    } else {
        Some(false)
    };
    return JsValue::from_serde(&status).unwrap();
}

fn compute_manifest_url(image: String) -> String {
    let label = "latest"; // TODO: extract from image path
    let mut protocol = "https://";
    let mut path = image.clone();
    if image.starts_with("http:") || image.starts_with("https:") {
        return image;
    }
    if !image.contains("/") {
        path = format!("library/{}", path);
    }
    if !image.contains(".") && !image.contains(":") {
        // No host
        path = format!("index.docker.io/{}", path);
        // N.B. actually this won't work because we need an authentication
        // token too (https://docs.docker.com/registry/spec/auth/token/#how-to-authenticate).
        // It's do-able, but not here yet...
    }
    path = path.replacen("/", "/v2/", 1);
    if path.starts_with("localhost") {
        protocol = "http://";
        // TODO: check for KUBERNETES env vars and 
        // path = path.replaceFirst("localhost", "registry");
    }
    let url = format!("{}{}/manifests/{}", protocol, path, label);
    return url;
}

fn info(value: String){
    unsafe { log(&value); }
}

async fn fetch_image_sha(path: String) -> Option<String> {
    let url = compute_manifest_url(path);
    let headers = serde_json::json!({
        "accept": "application/vnd.docker.distribution.manifest.v2+json"
    });
    unsafe {
        let result = getWithHeaders(url.as_str(), headers.to_string().as_str()).await;
        if result == JsValue::UNDEFINED {
            return None;
        } else {
            return extract_image(result);
        }
    }
}

fn extract_image(json: JsValue) -> Option<String> {
    let manifest: serde_json::Value = json.into_serde().ok()?;
    let headers = &manifest["headers"];
    if headers["Docker-Content-Digest"] != serde_json::json!(null) {
        return Some(headers["Docker-Content-Digest"].as_str()?.to_string());
    }
    if headers["docker-content-digest"] != serde_json::json!(null) {
        return Some(headers["docker-content-digest"].as_str()?.to_string());
    }
    return None;
}