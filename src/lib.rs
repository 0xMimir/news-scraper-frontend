mod error;
pub use error::Error;
pub mod services;
pub mod store;
pub mod app;
pub mod components;
pub mod routes;

use wasm_bindgen::prelude::*;

use app::App;

// Use `std::alloc` as the global allocator.
#[global_allocator]
static ALLOC: std::alloc::System = std::alloc::System;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}
