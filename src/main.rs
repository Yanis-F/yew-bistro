#[macro_use]
extern crate lazy_static;

mod app;
mod bistro;
mod eval;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
