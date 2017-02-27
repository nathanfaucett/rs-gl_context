extern crate prng;
extern crate rng;
extern crate time;
extern crate mat4;
extern crate gl_context;
extern crate glutin;
extern crate gl;


use std::f32::consts::PI;

use gl::types::*;
use gl_context::{Context, TextureKind, TextureFormat, TextureWrap, FilterMode};
use prng::Prng;
use rng::Rng;


static TO_RADS: f32 = PI / 180f32;

const TEX_WIDTH: usize = 512;
const TEX_HEIGHT: usize = TEX_WIDTH;
const TEX_SIZE: usize = TEX_WIDTH * TEX_HEIGHT;

static VS:  &'static str = "
    #version 140

    uniform mat4 projection;
    uniform mat4 model_view;

    uniform vec2 offset;
    uniform vec2 uv_offset;

    in vec2 position;
    in vec2 uv;

    out vec2 v_uv;

    void main() {
        gl_Position = projection * model_view * vec4(offset + position, 0.0, 1.0);
        v_uv = uv_offset + uv;
    }
";

static FS:  &'static str = "
    #version 140

    uniform sampler2D diffuse;

    in vec2 v_uv;

    out vec4 frag_color;

    void main() {
        frag_color = texture(diffuse, v_uv);
    }
";

static DATA: [GLfloat; 16] = [
    // vertices           uvs
    1f32,  1f32,   1f32, 1f32,
   -1f32,  1f32,   0f32, 1f32,
    1f32, -1f32,   1f32, 0f32,
   -1f32, -1f32,   0f32, 0f32
];

fn main() {
    let window = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build()
        .unwrap();

    let mut random = Prng::new();

    unsafe {
        match window.make_current() {
            Ok(_) => {
                gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            },
            Err(e) => panic!("{:?}", e),
        }
    }

    let mut context = Context::new();

    context.init();

    println!("{:?}", context.version());
    println!(
        "OpenGL version: {:?}.{:?}, GLSL version {:?}.{:?}0",
        context.major(), context.minor(), context.glsl_major(), context.glsl_minor()
    );

    let mut program = context.new_program();
    program.set(VS, FS);

    let mut data = [0xffffffffu32; TEX_SIZE];
    for i in 0..TEX_SIZE {
        let r = (random.next_f32() * 256f32) as u32;
        let c = (0xff000000) | (r << 16) | (r << 8) | r;
        data[i] = c;
    }

    let mut texture = context.new_texture();
    texture.set_data2d(
        &context,
        TEX_WIDTH,
        TEX_HEIGHT,
        TextureFormat::RGBA,
        TextureKind::UnsignedByte,
        TextureWrap::Repeat,
        FilterMode::None,
        true,
        &data
    );

    let vertex_array = context.new_vertex_array();
    context.set_vertex_array(&vertex_array, false);

    let mut buffer = context.new_buffer();
    buffer.set(gl::ARRAY_BUFFER, &DATA, 4, gl::STATIC_DRAW);

    context.remove_vertex_array(false);

    let mut perspective_matrix = mat4::new_identity::<f32>();
    let mut model_view = mat4::new_identity::<f32>();
    let camera = [0f32, 0f32, -5f32];
    let mut offset = [0f32, 0f32];
    let mut uv_offset = [0f32, 0f32];
    let mut color = [0f32, 0f32, 0f32, 1f32];
    let mut width = 512i32;
    let mut height = 512i32;

    let start_time = time::now();
    let mut last_time = start_time;
    let mut current_time;
    let mut ms = 0f32;
    let mut dt;

    let mut playing = true;

    while playing {
        current_time = time::now();
        dt = (current_time - last_time).num_nanoseconds().unwrap() as f32 * 0.000001f32;
        ms += dt;
        last_time = current_time;

        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => {
                    playing = false;
                },
                glutin::Event::Resized(w, h) => {
                    width = w as i32;
                    height = h as i32;
                    mat4::perspective(&mut perspective_matrix, 45f32 * TO_RADS, w as f32 / h as f32, 0.1f32, 1024f32);
                    context.set_viewport(0, 0, w as usize, h as usize);
                },
                glutin::Event::MouseMoved(x, y) => {
                    offset[0] = (((x - (width / 2)) as f32) / width as f32) * 2f32;
                    offset[1] = ((((height / 2) - y) as f32) / width as f32) * 2f32;
                },
                _ => (),
            }
        }

        color[0] = (ms * 0.000001f32).cos();
        color[1] = (ms * 0.0001f32).sin();
        color[2] = (ms * 0.001f32).cos();

        context.set_clear_color(&color);
        context.clear(true, true, true);

        uv_offset[0] = (ms * 0.0001f32).sin() * 0.5f32;
        uv_offset[1] = (ms * 0.0001f32).cos() * 0.5f32;

        mat4::set_position(&mut model_view, &camera);

        context.set_program(&program, false);

        context.set_vertex_array(&vertex_array, false);

        program.set_attribute("position", &mut context, &buffer, 0, false);
        program.set_attribute("uv", &mut context, &buffer, 2, false);

        program.set_uniform("diffuse", &mut context, &texture, false);
        program.set_uniform_unchecked("offset", &mut context, &offset, false);
        program.set_uniform_unchecked("uv_offset", &mut context, &uv_offset, false);
        program.set_uniform("projection", &mut context, &perspective_matrix, false);
        program.set_uniform_unchecked("model_view", &mut context, &model_view, false);

        unsafe { gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4); }

        match window.swap_buffers() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }
}
