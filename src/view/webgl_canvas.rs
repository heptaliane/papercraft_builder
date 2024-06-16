use web_sys::HtmlCanvasElement;
use yew::{function_component, html, use_effect_with, use_node_ref, Html};

use super::utils::{
    draw_triangles, get_webgl_rendering_context, rotate_view_point, set_color_data, set_ibo_data,
    set_vertex_data, setup_program, create_vertex_array,
};

const CANVAS_WIDTH: u32 = 800;
const CANVAS_HEIGHT: u32 = 800;

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
pub fn WebGLCanvas() -> Html {
    let canvas_ref = use_node_ref();

    {
        let canvas_ref = canvas_ref.clone();

        use_effect_with(canvas_ref, |canvas_ref| {
            let canvas = canvas_ref
                .cast::<HtmlCanvasElement>()
                .expect("canvas_ref not attached to canvas element");
            canvas.set_height(CANVAS_HEIGHT);
            canvas.set_width(CANVAS_WIDTH);

            draw_canvas(&canvas, &VERTICES, &COLORS, &INDICES, &[0.0, 0.0, 1.0], 0.0)
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
    let context = get_webgl_rendering_context(canvas).expect("Failed to get WebGL context");
    let program = setup_program(&context)?;

    let vao = create_vertex_array(&context)?;
    context.bind_vertex_array(Some(&vao));
    set_vertex_data(&context, &program, &vertex)?;
    set_color_data(&context, &program, &color)?;
    set_ibo_data(&context, &indices.concat())?;
    context.bind_vertex_array(Some(&vao));

    rotate_view_point(&context, &program, axis, radians);

    draw_triangles(&context, indices.len() as i32);

    Ok(())
}
