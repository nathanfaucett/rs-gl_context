extern crate gl;
extern crate glutin;
extern crate gl_context;


use gl::types::*;
use gl_context::{
    Context, TextureKind, TextureFormat, TextureWrap, FilterMode,
    Attachment, Usage, DrawMode, BufferTarget
};


static FB_VS:  &'static str = "
    #version 330 core

    in vec2 position;
    in vec2 uv;

    varying vec2 v_uv;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
        v_uv = uv;
    }
";
static FB_FS:  &'static str = "
    #version 330 core

    uniform sampler2D diffuse;

    in vec2 v_uv;

    out vec4 frag_color;

    void main() {
        frag_color = texture(diffuse, v_uv);
    }
";

static SIMPLE_VS: &'static str = "
    #version 330 core

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0, 1.0);
    }
";
static SIMPLE_FS: &'static str = "
    #version 330 core

    layout(location = 0) out vec4 out_color;

    void main() {
        out_color = vec4(1.0, 1.0, 1.0, 1.0);
    }
";


static FB_VERTEX_DATA: [GLfloat; 16] = [
    // vertices           uvs
     1f32,  1f32,   1f32, 1f32,
    -1f32,  1f32,   0f32, 1f32,
     1f32, -1f32,   1f32, 0f32,
    -1f32, -1f32,   0f32, 0f32
];
static TR_VERTEX_DATA: [GLfloat; 6] = [
    0.0,  0.5,
    -0.5, -0.5,
    0.5, -0.5
];


fn main() {
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build(&events_loop)
        .unwrap();

    let mut context = Context::new();

    unsafe {
        window.make_current()
    }.unwrap();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    context.init();

    println!("{:?}", context.version());
    println!(
        "OpenGL version: {:?}.{:?}, GLSL version {:?}.{:?}0",
        context.major(), context.minor(), context.glsl_major(), context.glsl_minor()
    );


    let mut fb_texture = context.new_texture();
    fb_texture.set_null2d(
        &context,
        256,
        256,
        TextureFormat::RGBA,
        TextureKind::UnsignedByte,
        TextureWrap::Clamp,
        FilterMode::None,
        false
    );


    let mut framebuffer = context.new_framebuffer();
    framebuffer.set(&context, &fb_texture, &[Attachment::Color], 0);

    let renderbuffer = context.new_renderbuffer();
    renderbuffer.set(&context, TextureFormat::DepthComponent, Attachment::Depth, 256, 256);


    let mut fb_program = context.new_program();
    fb_program.set(FB_VS, FB_FS);

    let fb_vertex_array = context.new_vertex_array();
    context.set_vertex_array(&fb_vertex_array, false);

    let mut fb_buffer = context.new_buffer();
    fb_buffer.set(BufferTarget::Array, &FB_VERTEX_DATA, 4, Usage::StaticDraw);


    let mut tr_program = context.new_program();
    tr_program.set(SIMPLE_VS, SIMPLE_FS);

    let tr_vertex_array = context.new_vertex_array();
    context.set_vertex_array(&tr_vertex_array, false);

    let mut tr_buffer = context.new_buffer();
    tr_buffer.set(BufferTarget::Array, &TR_VERTEX_DATA, 0, Usage::StaticDraw);


    let mut playing = true;
    let mut width = 512;
    let mut height = 512;
    static SIZE: usize = 8;
    while playing {

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event: glutin::WindowEvent::Closed, .. } => {
                    playing = false;
                },
                glutin::Event::WindowEvent { event: glutin::WindowEvent::Resized(w, h), .. } => {
                    width = w as usize;
                    height = h as usize;

                    fb_texture.set_null2d(
                        &context,
                        width / SIZE,
                        height / SIZE,
                        TextureFormat::RGBA,
                        TextureKind::UnsignedByte,
                        TextureWrap::Clamp,
                        FilterMode::None,
                        false
                    );
                    framebuffer.set(&context, &fb_texture, &[Attachment::Color], 0);
                    renderbuffer.set(&context, TextureFormat::DepthComponent, Attachment::Depth, width / SIZE, height / SIZE);
                },
                _ => (),
            }
        });

        context.set_framebuffer(&framebuffer, false);
        context.set_renderbuffer(&renderbuffer, false);

        context.set_viewport(0, 0, width / SIZE, height / SIZE);
        context.clear(true, true, true);
        context.set_clear_color(&[0.3, 0.3, 0.3, 1.0]);

        context.set_program(&tr_program, false);
        context.set_vertex_array(&tr_vertex_array, false);

        tr_program.set_attribute("position", &mut context, &tr_buffer, 0, false);

        context.draw_arrays(DrawMode::Triangles, 0, 3);


        context.remove_framebuffer(false);
        context.remove_renderbuffer(false);

        context.set_viewport(0, 0, width, height);
        context.clear(true, true, true);

        context.set_program(&fb_program, false);
        context.set_vertex_array(&fb_vertex_array, false);

        fb_program.set_attribute("position", &mut context, &fb_buffer, 0, false);
        fb_program.set_attribute("uv", &mut context, &fb_buffer, 2, false);
        fb_program.set_uniform("diffuse", &mut context, &fb_texture, false);

        context.draw_arrays(DrawMode::TriangleStrip, 0, 4);

        match window.swap_buffers() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }
}
