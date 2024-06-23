use std::cell::RefCell;
use std::rc::Rc;

use web_sys::HtmlCanvasElement;
use yew::{function_component, html, use_effect_with, use_node_ref, use_state, Html, Properties};

use super::utils::WebGlCanvasDrawer;

const DEFAULT_CANVAS_WIDTH: u32 = 800;
const DEFAULT_CANVAS_HEIGHT: u32 = 800;

const DEFAULT_AXIS: [f32; 3] = [0.0, 0.0, 1.0];
const DEFAULT_ANGLE: f32 = 0.0;

#[derive(Properties, PartialEq, Clone)]
pub struct WebGLCanvasProps {
    pub vertices: Rc<[[f32; 3]]>,
    pub colors: Rc<[[f32; 4]]>,
    pub indices: Rc<[[u16; 3]]>,

    #[prop_or(DEFAULT_CANVAS_HEIGHT)]
    pub height: u32,
    #[prop_or(DEFAULT_CANVAS_WIDTH)]
    pub width: u32,

    #[prop_or(Rc::from(DEFAULT_AXIS))]
    pub axis: Rc<[f32; 3]>,
    #[prop_or(DEFAULT_ANGLE)]
    pub rotate: f32,
}

#[function_component]
pub fn WebGLCanvas(props: &WebGLCanvasProps) -> Html {
    let canvas_ref = use_node_ref();
    let drawer_ref = use_state(|| Rc::new(RefCell::new(None::<WebGlCanvasDrawer>)));

    {
        let canvas_ref = canvas_ref.clone();
        let props = props.clone();
        let drawer_ref = drawer_ref.clone();

        use_effect_with(canvas_ref, move |canvas_ref| {
            let canvas = canvas_ref
                .cast::<HtmlCanvasElement>()
                .expect("canvas_ref not attached to canvas element");
            canvas.set_height(props.height);
            canvas.set_width(props.width);

            if let Ok(drawer) = WebGlCanvasDrawer::try_new(&canvas) {
                *drawer_ref.borrow_mut() = Some(drawer);
            }
        });
    }

    {
        let drawer_ref = drawer_ref.clone();
        let props = props.clone();
        use_effect_with(props, move |props| {
            if let Some(drawer) = drawer_ref.borrow_mut().as_mut() {
                drawer.set_drawing_data(
                    Vec::from(props.vertices.as_ref()),
                    Vec::from(props.colors.as_ref()),
                    &props.indices,
                );
                drawer.set_rotate_angle(props.axis.as_ref(), props.rotate);
                drawer.draw().expect("Failed to draw canvas");
            }
        });
    }

    html! {
        <canvas
            ref={canvas_ref}
        />
    }
}
