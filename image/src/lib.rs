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
    fn log(value: String);
}

#[wasm_bindgen]
pub async fn xform(json: String) -> JsValue {
    let image: V1Image = serde_json::from_str(&json).unwrap();
    let mut status = V1ImageStatus::new();
    status.complete = Some(false);
    match image.spec {
        Some(spec) => {
            match spec.image {
                Some(path) => {
                    let response = fetchImageSha(path.clone()).await;
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
    return JsValue::from_str(&serde_json::to_string(&status).unwrap());
}

fn computeManifestUrl(image: String) -> String {
    let label = "latest"; // TODO: extract from image path
    let mut protocol = "https://";
    let mut path = image.clone();
    if !image.contains("/") {
        path = format!("library/{}", path);
    }
    if !image.contains(".") && !image.contains(":") {
        // No host
        path = format!("index.docker.io/{}", path);
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

async fn fetchImageSha(path: String) -> Option<String> {
    let url = computeManifestUrl(path);
    let headers = serde_json::json!({
        "accept": "application/vnd.docker.distribution.manifest.v2+json"
    });
    let result = getWithHeaders(url.as_str(), headers.to_string().as_str()).await.as_string()?;
    if result.len()==0 {
        return None;
    } else {
        return extractImage(result);
    }
}

fn extractImage(json: String) -> Option<String> {
    if json.len()==0 {
        return None;
    }
    let manifest: serde_json::Value = serde_json::from_str(json.as_str()).ok()?;
    let headers = &manifest["headers"];
    if headers["Docker-Content-Digest"] != serde_json::json!(null) {
        return Some(headers["Docker-Content-Digest"].as_str()?.to_string());
    }
    if headers["docker-content-digest"] != serde_json::json!(null) {
        return Some(headers["docker-content-digest"].as_str()?.to_string());
    }
    return None;
}