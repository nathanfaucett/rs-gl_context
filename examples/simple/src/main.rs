extern crate gl;
extern crate glutin;
extern crate gl_context;


use gl::types::*;
use gl_context::{Context, Depth};


static VERTEX_DATA: [GLfloat; 6] = [
    0.0,  0.5,
    -0.5, -0.5,
    0.5, -0.5
];

static VS_SRC: &'static str = "
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0, 1.0);
    }
";

static FS_SRC: &'static str = "
    #version 140

    out vec4 out_color;

    void main() {
        out_color = vec4(1.0, 1.0, 1.0, 1.0);
    }
";

fn main() {
    let window = glutin::Window::new().unwrap();
    let mut context = Context::new();

    unsafe {
        match window.make_current() {
            Ok(_) => {
                gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            },
            Err(e) => panic!("{:?}", e),
        }
    }

    context.init();
    context.set_depth_func(Depth::Always);

    println!(
        "OpenGL version: {:?}.{:?}, GLSL version {:?}.{:?}0",
        context.get_major(), context.get_minor(), context.get_glsl_major(), context.get_glsl_minor()
    );

    let mut program = context.new_program();
    program.set(VS_SRC, FS_SRC);
    context.set_program(&program, false);

    let vertex_array = context.new_vertex_array();
    context.set_vertex_array(&vertex_array, false);

    let mut buffer = context.new_buffer();
    buffer.set(gl::ARRAY_BUFFER, &VERTEX_DATA, 0, gl::STATIC_DRAW);

    program.set_attribute("position", &mut context, &buffer, 0, false);

    let mut playing = true;
    while playing {
        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => {
                    playing = false;
                },
                glutin::Event::Resized(w, h) => {
                    context.set_viewport(0, 0, w as usize, h as usize);
                },
                _ => (),
            }
        }

        context.clear(true, true, true);
        context.set_clear_color(&[0.3, 0.3, 0.3, 1.0]);

        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3); }

        match window.swap_buffers() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }
}
