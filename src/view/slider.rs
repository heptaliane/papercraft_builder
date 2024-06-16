use wasm_bindgen::JsCast;
use yew::{function_component, html, Html, Properties, Callback};
use web_sys::{Event, HtmlInputElement};


const DEFAULT_MIN: f32 = 0.0;
const DEFAULT_MAX: f32 = 100.0;
const DEFAULT_STEP: f32 = 1.0;


#[derive(Properties, PartialEq)]
pub struct SliderProps {
    #[prop_or(DEFAULT_MAX)]
    pub max: f32,

    #[prop_or(DEFAULT_MIN)]
    pub min: f32,

    #[prop_or(DEFAULT_STEP)]
    pub step: f32,

    pub onchange: Callback<f32>,
}

#[function_component]
pub fn Slider(props: &SliderProps) -> Html {
    let handle_change = {
        let onchange = props.onchange.clone();
        Callback::from(move |e: Event| {
            let target = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(elem) = target {
                if let Ok(value) = elem.value().parse() {
                    onchange.emit(value);
                }
            }
        })
    };

    html! {
        <input
            type="range"
            max={props.max.to_string()}
            min={props.min.to_string()}
            step={props.step.to_string()}
            onchange={handle_change}
        />
    }
}
