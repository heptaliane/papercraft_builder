use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlVertexArrayObject,
};
use yew::{function_component, html, use_effect_with, use_node_ref, Html};

use crate::models::{Coord, CoordUnit};

const VERTEX_SHADER: &str = include_str!("./shader/vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("./shader/fragment.glsl");

const CANVAS_WIDTH: u32 = 800;
const CANVAS_HEIGHT: u32 = 800;

const VERTICES: &[Coord] = &[
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
            let context = create_webgl_context(&canvas).unwrap();

            let indices = INDICES.concat();
            let vao = create_vao(&context, &VERTICES, 0, &COLORS, 1, &indices).unwrap();
            context.bind_vertex_array(Some(&vao));
            draw(&context, indices.len() as i32);

            context.enable(WebGl2RenderingContext::DEPTH_TEST);
            context.depth_func(WebGl2RenderingContext::LEQUAL);
            context.enable(WebGl2RenderingContext::CULL_FACE);
        });
    }

    html! {
        <canvas
            ref={canvas_ref}
        />
    }
}

fn draw(context: &WebGl2RenderingContext, index_count: i32) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear_depth(1.0);
    context
        .clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    context.draw_elements_with_i32(
        WebGl2RenderingContext::TRIANGLES,
        index_count,
        WebGl2RenderingContext::UNSIGNED_SHORT,
        0,
    );
    context.flush();
}

fn create_webgl_context(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, JsValue> {
    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vertex_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        VERTEX_SHADER,
    )?;
    let fragment_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        FRAGMENT_SHADER,
    )?;
    let program = link_program(&context, &vertex_shader, &fragment_shader)?;
    context.use_program(Some(&program));

    Ok(context)
}

fn create_vao(
    context: &WebGl2RenderingContext,
    vertex: &[Coord],
    vertex_location: u32,
    color: &[[CoordUnit; 4]],
    color_location: u32,
    ibo_data: &[u16],
) -> Result<WebGlVertexArrayObject, String> {
    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));

    set_vbo_data(
        context,
        &(vertex.iter().map(|v| &v[..]).collect::<Vec<&[f32]>>()),
        vertex_location,
    )?;
    set_vbo_data(
        context,
        &(color.iter().map(|v| &v[..]).collect::<Vec<&[f32]>>()),
        color_location,
    )?;
    set_ibo_data(context, &ibo_data)?;

    context.bind_vertex_array(None);

    Ok(vao)
}

fn set_vbo_data(
    context: &WebGl2RenderingContext,
    data: &[&[f32]],
    location: u32,
) -> Result<(), String> {
    let flatten_data = data.concat();
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let view = js_sys::Float32Array::view(&flatten_data);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    context.enable_vertex_attrib_array(location);
    let size = flatten_data.len() / data.len();
    context.vertex_attrib_pointer_with_i32(
        location,
        size as i32,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    Ok(())
}

fn set_ibo_data(context: &WebGl2RenderingContext, data: &[u16]) -> Result<(), String> {
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let view = js_sys::Uint16Array::view(&data);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    Ok(())
}

fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unkown error creating shader")))
    }
}

fn link_program(
    context: &WebGl2RenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vertex_shader);
    context.attach_shader(&program, fragment_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
