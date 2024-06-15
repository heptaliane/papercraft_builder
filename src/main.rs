mod models;
mod transform;
mod view;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<view::app::App>::new().render();
}
