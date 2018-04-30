#![feature(proc_macro)]
#![feature(wasm_custom_section)]
#![feature(wasm_import_module)]

#[macro_use]
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn foo() {}
