pub mod helpers;

pub use helpers::error::Error;

pub mod context;
pub mod store;
pub mod app;
pub mod components;
pub mod routes;

use app::App;

pub fn main(){
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
