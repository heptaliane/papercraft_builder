use std::rc::Rc;

use yew::prelude::{function_component, html, use_state, Html};
use yew::Callback;

use super::slider::Slider;
use super::webgl_canvas::WebGLCanvas;

const VERTICES: &[[f32; 3]] = &[
    [0.0, 0.0, 0.0],
    [0.5, 0.0, 0.0],
    [0.5, 0.5, 0.0],
    [0.0, 0.5, 0.0],
];

const COLORS: &[[f32; 4]] = &[
    [1.0, 0.0, 0.0, 1.0],
    [1.0, 0.0, 0.0, 1.0],
    [1.0, 0.0, 0.0, 1.0],
    [1.0, 0.0, 0.0, 1.0],
];

const INDICES: &[[u16; 3]] = &[[0, 1, 2], [0, 2, 3]];

#[function_component]
pub fn App() -> Html {
    let angle = use_state(|| 0.0_f32);
    let handle_change = Callback::from({
        let angle = angle.clone();
        move |v: f32| {
            angle.set(v);
        }
    });

    html! {
        <div>
            <WebGLCanvas
                vertices={Rc::from(VERTICES)}
                colors={Rc::from(COLORS)}
                indices={Rc::from(INDICES)}
                rotate={(*angle).clone() / 180.0 * std::f32::consts::PI}
            />
            <Slider
                max={180.0}
                min={-180.0}
                onchange={handle_change}
            />
            {(*angle).clone()}
        </div>
    }
}
