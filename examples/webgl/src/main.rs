#![feature(link_args)]


#[link_args = "-s USE_SDL=2"]
extern {}


#[macro_use]
extern crate webplatform;
extern crate gl;
extern crate sdl2;
extern crate gl_context;


use gl::types::*;
use gl_context::Context;


static VERTEX_DATA: [GLfloat; 6] = [
    0.0,  0.5,
    -0.5, -0.5,
    0.5, -0.5
];

static VS_SRC: &'static str = "
    attribute vec2 position;

    void main() {
        gl_Position = vec4(position, 0, 1.0);
    }
";

static FS_SRC: &'static str = "
    void main() {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
";

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" ||
           item.name == "opengles"||
           item.name == "opengles2" {
            return Some(index as u32);
        }
    }
    None
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    video_subsystem.gl_attr().set_depth_size(24);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut width = 960;
    let mut height = 640;

    let window = video_subsystem.window("WebGL", width, height)
        .resizable()
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window with given parameters");
    let renderer = window.renderer()
        .present_vsync()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .expect("Failed to create renderer with given parameters");

    let mut context = Context::new();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    renderer.window().unwrap().gl_set_context_to_current().unwrap();

    context.init();

    println!(
        "OpenGL version: {:?}.{:?}, GLSL version {:?}.{:?}0",
        context.major(), context.minor(), context.glsl_major(), context.glsl_minor()
    );
    let mut program = context.new_program();
    program.set(VS_SRC, FS_SRC);
    context.set_program(&program, false);

    let vertex_array = context.new_vertex_array();
    context.set_vertex_array(&vertex_array, false);

    let mut buffer = context.new_buffer();
    buffer.set(gl::ARRAY_BUFFER, &VERTEX_DATA, 0, gl::STATIC_DRAW);

    program.set_attribute("position", &mut context, &buffer, 0, false);

    context.set_viewport(0, 0, width as usize, height as usize);

    for event in event_pump.poll_iter() {
        match event {
            sdl2::event::Event::Quit{..} => {},
            _ => (),
        }
    }

    {
       let window = renderer.window().unwrap();
       let size = window.size();

       if size.0 != width || size.1 != height {
           width = size.0;
           height = size.1;
           context.set_viewport(0, 0, width as usize, height as usize);
       }
   }

    context.clear(true, true, true);
    context.set_clear_color(&[0.3, 0.3, 0.3, 1.0]);

    unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3); }

    renderer.window().unwrap().gl_swap_window();

    let _ = webplatform::init();
    webplatform::spin();
}
