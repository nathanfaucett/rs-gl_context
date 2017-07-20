#![feature(drop_types_in_const)]
#![feature(libc)]


extern crate gl;
extern crate glutin;
extern crate gl_context;
#[macro_use]
extern crate webplatform;
extern crate time;
extern crate libc;


use std::rc::Rc;
use std::cell::RefCell;

use glutin::GlContext;
use gl::types::*;
use gl_context::{Program, VertexArray, Buffer, Context};


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


pub struct App {
    events_loop: RefCell<glutin::EventsLoop>,
    window: glutin::Window,
    context: RefCell<Context>,
    program: Program,
    vertex_array: VertexArray,
    buffer: Buffer,
    width: usize,
    height: usize,
    ms: f64,
    last_time: time::Tm,
}

impl App {
    pub fn new() -> Self {
        let width = 960usize;
        let height = 640usize;

        let mut events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title("Simple")
            .with_dimensions(width, height);
        let ctx = glutin::ContextBuilder::new()
            .with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, ctx, &events_loop).unwrap();

        let mut context = Context::new();

        unsafe {
            gl_window.make_current().unwrap();
        }

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

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

        App {
            events_loop: RefCell::new(events_loop),
            window: window,
            context: RefCell::new(context),
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
        //let (w, h) = {
        //    let window = self.renderer.window().unwrap();
        //    let size = window.size();
        //    (size.0 as usize, size.1 as usize)
        //};

        //if w != self.width || h != self.height {
            //self.set_size(w, h);
        //}
    }
    fn set_size(&mut self, w: usize, h: usize) {
        //let _ = self.renderer.window_mut().unwrap().set_size(w as u32, h as u32);
        self.width = w;
        self.height = h;
        self.context.borrow_mut().set_viewport(0, 0, w, h);
    }

    pub fn update(&mut self) {
        let events_loop = self.events_loop.borrow();
        let mut context = self.context.borrow_mut();

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event: glutin::WindowEvent::Closed, .. } => {},
                glutin::Event::WindowEvent { event: glutin::WindowEvent::Resized(w, h), .. } => {
                    //context.set_viewport(0, 0, w as usize, h as usize);
                },
                _ => (),
            }
        });

        context.clear(true, true, true);
        context.set_clear_color(&[0.3, 0.3, 0.3, 1.0]);

        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3); }

        gl_window.swap_buffers().unwrap();
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
