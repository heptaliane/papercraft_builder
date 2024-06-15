use yew::prelude::{function_component, html, Html};

use crate::view::webgl_canvas::WebGLCanvas;

#[function_component]
pub fn App() -> Html {
    html! {
        <WebGLCanvas />
    }
}
