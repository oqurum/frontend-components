use gloo_utils::body;
use wasm_bindgen::prelude::*;

mod app;
mod components;
mod routes;

pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    body().set_class_name("text-light d-flex flex-column");

    yew::Renderer::<app::App>::new().render();

    log::debug!("Rendering App");

    Ok(())
}
