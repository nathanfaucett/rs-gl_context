#![feature(libc)]
#![feature(drop_types_in_const)]
#![feature(link_args)]


#[link_args = "-s USE_SDL=2"]
extern {}


extern crate libc;
#[macro_use]
extern crate webplatform;
extern crate time;

extern crate gl;
extern crate sdl2;
extern crate gl_context;


use std::rc::Rc;
use std::cell::RefCell;

use gl::types::*;
use gl_context::{Program, VertexArray, Buffer, Context};


static VERTEX_DATA: [GLfloat; 6] = [
    0.0,  0.5,
    -0.5, -0.5,
    0.5, -0.5
];

static VS_SRC: &'static str = "
    attribute vec2 position;
    uniform vec2 offset;

    void main() {
        gl_Position = vec4(offset + position, 0, 1.0);
    }
";

static FS_SRC: &'static str = "
    void main() {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
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


pub struct App<'a> {
    event_pump: sdl2::EventPump,
    renderer: sdl2::render::Renderer<'a>,
    context: Context,
    program: Program,
    vertex_array: VertexArray,
    buffer: Buffer,
    width: usize,
    height: usize,
    ms: f64,
    last_time: time::Tm,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let width = 960usize;
        let height = 640usize;
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        video_subsystem.gl_attr().set_depth_size(24);

        let window = video_subsystem.window("WebGL", width as u32, height as u32)
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

        println!("{:?}", context.version());
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

        context.set_viewport(0, 0, width, height);

        App {
            event_pump: event_pump,
            renderer: renderer,
            context: context,
            program: program,
            vertex_array: vertex_array,
            buffer: buffer,
            width: width,
            height: height,
            ms: 0f64,
            last_time: time::now(),
        }
    }

    pub fn resize(&mut self) {
        let (w, h) = {
            let window = self.renderer.window().unwrap();
            let size = window.size();
            (size.0 as usize, size.1 as usize)
        };

        if w != self.width || h != self.height {
            self.set_size(w, h);
        }
    }
    fn set_size(&mut self, w: usize, h: usize) {
        let _ = self.renderer.window_mut().unwrap().set_size(w as u32, h as u32);
        self.width = w;
        self.height = h;
        self.context.set_viewport(0, 0, w, h);
    }

    pub fn update(&mut self) {
        let current_time = time::now();
        let dt = (current_time - self.last_time).num_nanoseconds().unwrap() as f64 * 0.000001f64;

        self.ms += dt;
        self.last_time = current_time;

        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit{..} => {},
                _ => (),
            }
        }

        self.resize();

        self.context.clear(true, true, true);
        self.context.set_clear_color(&[0.3, 0.3, 0.3, 1.0]);

        let offset = [
            ((self.ms * 0.001f64).sin() * 0.5f64) as f32,
            ((self.ms * 0.001f64).cos() * 0.5f64) as f32,
        ];
        self.program.set_uniform_unchecked("offset", &mut self.context, &offset, false);

        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3); }

        self.renderer.window().unwrap().gl_swap_window();
    }
}


static mut APP: Option<Rc<RefCell<App>>> = None;


extern "C" fn main_loop() {
    unsafe {
        APP.as_mut().unwrap().borrow_mut().update();
    }
}

fn main() {
    let mut app = unsafe {
        APP = Some(Rc::new(RefCell::new(App::new())));
        APP.clone()
    };

    let document = webplatform::init();

    let mut on_resize = move |_: webplatform::Event| {
        let w = js! { b"\
            return window.innerWidth;\
        \0" } as usize;
        let h = js! { b"\
            return window.innerHeight;\
        \0" } as usize;

        app.as_mut().unwrap().borrow_mut().set_size(w, h);
    };

    on_resize(webplatform::Event {
        target: None,
    });
    document.on("resize", on_resize);

    unsafe {
        webplatform::emscripten_set_main_loop(main_loop, 0, 1);
    }

    webplatform::spin();
}
