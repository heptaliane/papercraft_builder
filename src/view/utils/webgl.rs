use nalgebra_glm as glm;
use wasm_bindgen::prelude::{JsCast, JsValue};
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlVertexArrayObject,
};

const VERTEX_SHADER: &str = include_str!("./shader/vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("./shader/fragment.glsl");

const VERTEX_SHADER_ATTRIB_POSITION: &str = "position";
const VERTEX_SHADER_ATTRIB_COLOR: &str = "color";
const VERTEX_SHADER_UNIFORM_VIEW_MATRIX: &str = "viewMatrix";

const CAMERA_POS: [f32; 3] = [0.0, 0.0, 1.0];
const CAMERA_TARGET: [f32; 3] = [0.0, 0.0, 0.0];
const CAMERA_UP: [f32; 3] = [0.0, 1.0, 0.0];

pub fn get_webgl_rendering_context(
    canvas: &HtmlCanvasElement,
) -> Result<WebGl2RenderingContext, JsValue> {
    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    context.enable(WebGl2RenderingContext::DEPTH_TEST);
    context.depth_func(WebGl2RenderingContext::LEQUAL);
    context.enable(WebGl2RenderingContext::CULL_FACE);

    Ok(context)
}

pub fn setup_program(context: &WebGl2RenderingContext) -> Result<WebGlProgram, String> {
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

    Ok(program)
}

pub fn setup_vertex_array(
    context: &WebGl2RenderingContext,
) -> Result<(), String> {
    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));
    Ok(())
}

pub fn set_vertex_data(
    context: &WebGl2RenderingContext,
    program: &WebGlProgram,
    vertex_data: &[[f32; 3]],
) -> Result<(), String> {
    let location = context.get_attrib_location(program, VERTEX_SHADER_ATTRIB_POSITION) as u32;
    set_vbo_data(context, vertex_data, location)?;
    Ok(())
}

pub fn set_color_data(
    context: &WebGl2RenderingContext,
    program: &WebGlProgram,
    color_data: &[[f32; 4]],
) -> Result<(), String> {
    let location = context.get_attrib_location(program, VERTEX_SHADER_ATTRIB_COLOR) as u32;
    set_vbo_data(context, color_data, location)?;
    Ok(())
}

pub fn set_ibo_data(context: &WebGl2RenderingContext, data: &[u16]) -> Result<(), String> {
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

pub fn rotate_view_point(
    context: &WebGl2RenderingContext,
    program: &WebGlProgram,
    axis: &[f32; 3],
    radians: f32,
) {
    let camera_pos = glm::make_vec3(&CAMERA_POS);
    let camera_target = glm::make_vec3(&CAMERA_TARGET);
    let camera_up = glm::make_vec3(&CAMERA_UP);
    let view_matrix = glm::look_at(&camera_pos, &camera_target, &camera_up);

    let axis = glm::make_vec3(axis);
    let rorate_matrix = glm::rotate(&glm::Mat4::identity(), radians, &axis);

    // let glm_matrix = view_matrix * rorate_matrix;
    let glm_matrix = glm::Mat4::identity();
    let matrix: [[f32; 4]; 4] = glm_matrix.into();
    let glsl_matrix = matrix.concat().to_vec();

    let view_location = context.get_uniform_location(program, VERTEX_SHADER_UNIFORM_VIEW_MATRIX);
    context.uniform_matrix4fv_with_f32_array_and_src_offset_and_src_length(
        view_location.as_ref(),
        false,
        &glsl_matrix,
        0,
        0,
    );
}

pub fn draw_triangles(context: &WebGl2RenderingContext, index_count: i32) {
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

fn set_vbo_data<const N: usize>(
    context: &WebGl2RenderingContext,
    data: &[[f32; N]],
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
    context.vertex_attrib_pointer_with_i32(
        location,
        N as i32,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    Ok(())
}
