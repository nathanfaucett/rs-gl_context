extern crate time;
extern crate mat4;
extern crate gl_context;
extern crate glutin;
extern crate gl;


use std::f32::consts::PI;

use glutin::Window;
use gl_context::Context;


static TO_RADS: f32 = PI / 180f32;


fn main() {
    let window = Window::new().unwrap();

    unsafe {
        window.make_current();
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    }

    let mut context = Context::new();

    let vertex = String::from("
        #version 150

        uniform mat4 projection;
        uniform mat4 model_view;

        uniform vec2 offset;

        attribute vec3 position;
        attribute vec2 uv;

        varying vec2 v_uv;

        void main() {
            gl_Position = projection * model_view * vec4(position, 1.0);
            v_uv = offset + uv;
        }
    ");

    let fragment = String::from("
        #version 150

        out vec4 frag_color;

        varying vec2 v_uv;

        void main() {
            frag_color = vec4(v_uv, 1.0, 1.0);
        }
    ");

    println!(
        "OpenGL version: {:?}.{:?}, GLSL version {:?}.{:?}0",
        context.major(), context.minor(), context.glsl_major(), context.glsl_minor()
    );

    let mut program = context.new_program();
    program.set(vertex, fragment);

    let vertex_array = context.new_vertex_array();
    context.set_vertex_array(&vertex_array);

    let mut buffer = context.new_buffer();
    buffer.set(gl::ARRAY_BUFFER, &[
        // vertices           uvs
        1f32, 1f32, 0f32,     0f32, 0f32,
        -1f32, 1f32, 0f32,    1f32, 0f32,
        1f32, -1f32, 0f32,    0f32, 1f32,
        -1f32, -1f32, 0f32,   1f32, 1f32
    ], 5, gl::STATIC_DRAW);

    context.remove_vertex_array();

    let mut perspective_matrix = mat4::new_identity::<f32>();
    let mut model_view = mat4::new_identity::<f32>();
    let mut camera = [0f32, 0f32, -5f32];
    let mut offset = [0f32, 0f32];
    let mut color = [0f32, 0f32, 0f32, 1f32];

    let start_time = time::now();
    let mut ms;

    let mut playing = true;

    while playing {
        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => {
                    playing = false;
                },
                glutin::Event::Resized(w, h) => {
                    mat4::perspective(&mut perspective_matrix, 45f32 * TO_RADS, w as f32 / h as f32, 0.1f32, 1024f32);
                    context.set_viewport(0, 0, w as usize, h as usize);
                },
                _ => (),
            }
        }

        ms = (time::now() - start_time).num_nanoseconds().unwrap() as f32 * 0.000001f32;

        color[0] = (ms * 0.000001f32).sin();
        color[1] = (ms * 0.0001f32).cos();
        color[2] = (ms * 0.001f32).sin();

        context.set_clear_color(color);
        context.clear(true, true, true);

        camera[0] = ((ms * 0.001f32) * 2f32).sin();
        camera[1] = 0f32;

        offset[0] = ((ms * 0.01f32) * 0.5f32).sin();
        offset[1] = ((ms * 0.01f32) * 0.5f32).cos();

        context.set_program(&program, false);

        mat4::set_position(&mut model_view, camera);

        context.set_vertex_array(&vertex_array);

        program.set_attribute(String::from("position"), &mut context, &buffer, 0, false);
        program.set_attribute(String::from("uv"), &mut context, &buffer, 3, false);

        program.set_uniform_unchecked(String::from("offset"), &mut context, &offset, false);
        program.set_uniform(String::from("projection"), &mut context, &perspective_matrix, false);
        program.set_uniform_unchecked(String::from("model_view"), &mut context, &model_view, false);

        unsafe { gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4); }

        window.swap_buffers();
    }
}
