#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::collections::BTreeMap;

use std::str;

use k8s_openapi::api::apps::v1::*;
use k8s_openapi::api::core::v1::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::*;

use wasm_bindgen::prelude::wasm_bindgen;

mod models;

use models::*;

#[wasm_bindgen]
pub fn xform(json: &str) -> String {
    let mut image: V1Image = serde_json::from_str(json).unwrap();
    return serde_json::to_string(&image).unwrap();
}