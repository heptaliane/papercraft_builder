use std::rc::Rc;

use web_sys::HtmlCanvasElement;
use yew::{function_component, html, use_effect_with, use_node_ref, Html, Properties};

use super::utils::{
    draw_triangles, get_webgl_rendering_context, rotate_view_point, set_color_data, set_ibo_data,
    set_vertex_data, setup_program, setup_vertex_array,
};

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

    {
        let canvas_ref = canvas_ref.clone();
        let props = props.clone();

        use_effect_with(canvas_ref, move |canvas_ref| {
            let canvas = canvas_ref
                .cast::<HtmlCanvasElement>()
                .expect("canvas_ref not attached to canvas element");
            canvas.set_height(props.height);
            canvas.set_width(props.width);

            draw_canvas(
                &canvas,
                &props.vertices,
                &props.colors,
                &props.indices,
                &props.axis,
                props.rotate,
            )
            .expect("Failed to WebGL render");
        });
    }

    html! {
        <canvas
            ref={canvas_ref}
        />
    }
}

fn draw_canvas(
    canvas: &HtmlCanvasElement,
    vertex: &[[f32; 3]],
    color: &[[f32; 4]],
    indices: &[[u16; 3]],
    axis: &[f32; 3],
    radians: f32,
) -> Result<(), String> {
    let flatten_indices = indices.concat();

    let context = get_webgl_rendering_context(canvas).expect("Failed to get WebGL context");
    let program = setup_program(&context)?;

    setup_vertex_array(&context)?;
    set_vertex_data(&context, &program, &vertex)?;
    set_color_data(&context, &program, &color)?;
    set_ibo_data(&context, &flatten_indices)?;

    rotate_view_point(&context, &program, axis, radians);

    draw_triangles(&context, flatten_indices.len() as i32);

    Ok(())
}
