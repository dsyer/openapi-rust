mod utils;

use openapi::models::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn xform(json: &str) -> String {
    let mut deployment: IoK8sApiAppsV1Deployment = serde_json::from_str(json).unwrap();
    deployment.api_version = match deployment.api_version {
        Some(value) => Some(value),
        None => Some("apps/v1".to_string()),
    };
    deployment.kind = match deployment.kind {
        Some(value) => Some(value),
        None => Some("Deployment".to_string()),
    };
    deployment.metadata = if let Some(value) = deployment.metadata {
        Some(value)
    } else {
        Some(
            IoK8sApimachineryPkgApisMetaV1ObjectMeta {
                annotations: None,
                cluster_name: None,
                creation_timestamp: None,
                deletion_grace_period_seconds: None,
                deletion_timestamp: None,
                finalizers: None,
                generate_name: None,
                generation: None,
                labels: None,
                managed_fields: None,
                name: None,
                namespace: None,
                owner_references: None,
                resource_version: None,
                self_link: None,
                uid: None,
            }
            .into(),
        )
    };
    return serde_json::to_string(&deployment).unwrap();
}
