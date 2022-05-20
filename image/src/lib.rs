#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use wasm_bindgen::prelude::wasm_bindgen;

mod models;

use models::*;

#[wasm_bindgen]
pub fn xform(json: &str) -> String {
    let image: V1Image = serde_json::from_str(json).unwrap();
    let mut status = V1ImageStatus::new();
    status.complete = Some(false);
    return serde_json::to_string(&status).unwrap();
}