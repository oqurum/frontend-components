use wasm_bindgen::prelude::*;

mod app;
mod components;
mod routes;

pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    yew::start_app::<app::App>();

    Ok(())
}
