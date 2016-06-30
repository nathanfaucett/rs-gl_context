use collections::vec::Vec;

use gl;
use gl::types::*;
use regex::Regex;
use enums::*;
use buffer::Buffer;
use program::Program;
use texture::Texture;
use vertex_array::VertexArray;


static HIGHP: &'static str = "highp";
static MEDIUMP: &'static str = "mediump";
static LOWP: &'static str = "lowp";


#[derive(Debug)]
pub struct Context {
    version_string: String,

    major: usize,
    minor: usize,

    extenstions: Vec<String>,

    clear_color: [f32; 4],

    max_anisotropy: usize,
    max_textures: usize,
    max_vertex_textures: usize,
    max_texture_size: usize,
    max_cube_texture_size: usize,
    max_render_buffer_size: usize,

    max_uniforms: usize,
    max_varyings: usize,
    max_attributes: usize,

    enabled_attributes: Vec<bool>,

    viewport_x: usize,
    viewport_y: usize,
    viewport_width: usize,
    viewport_height: usize,

    blending: Blending,
    cull_face: CullFace,
    depth_func: Depth,

    blending_disabled: bool,
    cull_face_disabled: bool,
    depth_test_disabled: bool,
    depth_write: bool,
    line_width: f32,

    current_array_buffer: usize,
    current_element_array_buffer: usize,

    current_vertex_array: usize,

    program: usize,
    precision: &'static str,
    program_force: bool,

    texture_index: usize,
    active_index: isize,
    active_texture: usize,
}

impl Context {

    pub fn new() -> Self {
        let mut context = Context {
            version_string: String::new(),

            major: 0,
            minor: 0,

            extenstions: Vec::new(),

            clear_color: [0f32, 0f32, 0f32, 1f32],

            max_anisotropy: 0,
            max_textures: 0,
            max_vertex_textures: 0,
            max_texture_size: 0,
            max_cube_texture_size: 0,
            max_render_buffer_size: 0,

            max_uniforms: 0,
            max_varyings: 0,
            max_attributes: 0,

            enabled_attributes: Vec::new(),

            viewport_x: 0,
            viewport_y: 0,
            viewport_width: 1,
            viewport_height: 1,

            blending: Blending::Default,
            cull_face: CullFace::Back,
            depth_func: Depth::LessThan,

            blending_disabled: true,
            cull_face_disabled: true,
            depth_test_disabled: true,
            depth_write: false,
            line_width: 1f32,

            current_array_buffer: 0,
            current_element_array_buffer: 0,

            current_vertex_array: 0,

            program: 0,
            precision: HIGHP,
            program_force: false,

            texture_index: 0,
            active_index: -1,
            active_texture: 0,
        };

        context.get_gl_info();
        context.gl_reset();

        context
    }

    pub fn major(&self) -> usize { self.major }
    pub fn minor(&self) -> usize { self.minor }

    pub fn reset(&mut self) -> &mut Self {

        self.version_string.clear();
        self.extenstions.clear();

        self.clear_color[0] = 0f32;
        self.clear_color[1] = 0f32;
        self.clear_color[2] = 0f32;
        self.clear_color[3] = 1f32;

        self.max_anisotropy = 0;
        self.max_textures = 0;
        self.max_vertex_textures = 0;
        self.max_texture_size = 0;
        self.max_cube_texture_size = 0;
        self.max_render_buffer_size = 0;

        self.max_uniforms = 0;
        self.max_varyings = 0;
        self.max_attributes = 0;

        self.enabled_attributes.clear();

        self.viewport_x = 0;
        self.viewport_y = 0;
        self.viewport_width = 1;
        self.viewport_height = 1;

        self.blending = Blending::Default;
        self.cull_face = CullFace::Back;
        self.depth_func = Depth::LessThan;

        self.blending_disabled = true;
        self.cull_face_disabled = true;
        self.depth_test_disabled = true;
        self.depth_write = false;
        self.line_width = 1f32;

        self.current_array_buffer = 0;
        self.current_element_array_buffer = 0;

        self.program = 0;
        self.precision = HIGHP;
        self.program_force = false;

        self.texture_index = 0;
        self.active_index = -1;
        self.active_texture = 0;

        self.get_gl_info();
        self.gl_reset()
    }

    fn gl_reset(&mut self) -> &mut Self {

        self.disable_attributes();

        self.set_viewport_unchecked(self.viewport_x, self.viewport_y, self.viewport_width, self.viewport_height);

        self.set_depth_write_unchecked(self.depth_write);
        self.set_line_width_unchecked(self.line_width);

        let blending = self.blending;
        let cull_face = self.cull_face;
        let depth_func = self.depth_func;
        self.set_blending_unchecked(blending);
        self.set_cull_face_unchecked(cull_face);
        self.set_depth_func_unchecked(depth_func);

        self.set_clear_color_unchecked(self.clear_color);
        self.clear(true, true, true);

        self
    }

    #[inline(always)]
    pub fn set_viewport_unchecked(&self, x: usize, y: usize, width: usize, height: usize) -> &Self {
        unsafe { gl::Viewport(x as i32, y as i32, width as i32, height as i32); }
        self
    }
    pub fn set_viewport(&mut self, x: usize, y: usize, width: usize, height: usize) -> &mut Self {
        if
            self.viewport_x != x ||
            self.viewport_y != y ||
            self.viewport_width != width ||
            self.viewport_height != height
        {
            self.viewport_x = x;
            self.viewport_y = y;
            self.viewport_width = width;
            self.viewport_height = height;
            self.set_viewport_unchecked(x, y, width, height);
        }
        self
    }

    #[inline(always)]
    pub fn set_depth_write_unchecked(&self, depth_write: bool) -> &Self {
        unsafe { gl::DepthMask(if depth_write { 1 } else { 0 }); }
        self
    }
    pub fn set_depth_write(&mut self, depth_write: bool) -> &mut Self {
        if self.depth_write != depth_write {
            self.depth_write = depth_write;
            self.set_depth_write_unchecked(depth_write);
        }
        self
    }

    #[inline(always)]
    pub fn set_line_width_unchecked(&self, line_width: f32) -> &Self {
        unsafe { gl::LineWidth(line_width); }
        self
    }
    pub fn set_line_width(&mut self, line_width: f32) -> &mut Self {
        if self.line_width != line_width {
            self.line_width = line_width;
            self.set_line_width_unchecked(line_width);
        }
        self
    }

    #[inline(always)]
    pub fn set_blending_unchecked(&mut self, blending: Blending) -> &mut Self {
        if self.blending_disabled {
            unsafe { gl::Enable(gl::BLEND); }
            self.blending_disabled = false;
        }

        match blending {
            Blending::Additive => {
                unsafe {
                    gl::BlendEquation(gl::FUNC_ADD);
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
                }
            },
            Blending::Subtractive => {
                unsafe {
                    gl::BlendEquation(gl::FUNC_ADD);
                    gl::BlendFunc(gl::ZERO, gl::ONE_MINUS_SRC_COLOR);
                }
            },
            Blending::Multiply => {
                unsafe {
                    gl::BlendEquation(gl::FUNC_ADD);
                    gl::BlendFunc(gl::ZERO, gl::SRC_COLOR);
                }
            },
            Blending::Default => {
                unsafe {
                    gl::BlendEquationSeparate(gl::FUNC_ADD, gl::FUNC_ADD);
                    gl::BlendFuncSeparate(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA, gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
                }
            },
            Blending::None => {
                unsafe { gl::Disable(gl::BLEND); }
                self.blending_disabled = true;
            },
        }
        self
    }
    pub fn set_blending(&mut self, blending: Blending) -> &mut Self {
        if self.blending != blending {
            self.blending = blending;
            self.set_blending_unchecked(blending);
        }
        self
    }

    #[inline(always)]
    pub fn set_cull_face_unchecked(&mut self, cull_face: CullFace) -> &mut Self {
        if self.cull_face_disabled {
            unsafe { gl::Enable(gl::CULL_FACE); }
            self.cull_face_disabled = false;
        }

        match cull_face {
            CullFace::Back => {
                unsafe { gl::CullFace(gl::BACK); }
            },
            CullFace::Front => {
                unsafe { gl::CullFace(gl::FRONT); }
            },
            CullFace::FrontAndBack => {
                unsafe { gl::CullFace(gl::FRONT_AND_BACK); }
            },
            CullFace::None => {
                self.cull_face_disabled = true;
                unsafe { gl::Disable(gl::CULL_FACE); }
            },
        }
        self
    }
    pub fn set_cull_face(&mut self, cull_face: CullFace) -> &mut Self {
        if self.cull_face != cull_face {
            self.cull_face = cull_face;
            self.set_cull_face_unchecked(cull_face);
        }
        self
    }

    #[inline(always)]
    pub fn set_depth_func_unchecked(&mut self, depth_func: Depth) -> &mut Self {
        if self.depth_test_disabled {
            unsafe { gl::Enable(gl::DEPTH_TEST); }
            self.depth_test_disabled = false;
        }

        match depth_func {
            Depth::Never => {
                unsafe { gl::DepthFunc(gl::NEVER); }
            },
            Depth::LessThan => {
                unsafe { gl::DepthFunc(gl::LESS); }
            },
            Depth::Equal => {
                unsafe { gl::DepthFunc(gl::EQUAL); }
            },
            Depth::LessThanOrEqual => {
                unsafe { gl::DepthFunc(gl::LEQUAL); }
            },
            Depth::GreaterThan => {
                unsafe { gl::DepthFunc(gl::GREATER); }
            },
            Depth::NotEqual => {
                unsafe { gl::DepthFunc(gl::NOTEQUAL); }
            },
            Depth::GreaterThanOrEqual => {
                unsafe { gl::DepthFunc(gl::GEQUAL); }
            },
            Depth::Always => {
                unsafe { gl::DepthFunc(gl::ALWAYS); }
            },
            Depth::None => {
                self.depth_test_disabled = true;
                unsafe { gl::Disable(gl::DEPTH_TEST); }
            },
        }
        self
    }
    pub fn set_depth_func(&mut self, depth_func: Depth) -> &mut Self {
        if self.depth_func != depth_func {
            self.depth_func = depth_func;
            self.set_depth_func_unchecked(depth_func);
        }
        self
    }

    #[inline(always)]
    pub fn set_clear_color_unchecked(&self, color: [f32; 4]) -> &Self {
        unsafe { gl::ClearColor(color[0], color[1], color[2], color[3]); }
        self
    }
    pub fn set_clear_color(&mut self, color: [f32; 4]) -> &mut Self {
        if self.clear_color != color {
            self.clear_color = color;
            self.set_clear_color_unchecked(color);
        }
        self
    }

    pub fn clear(&mut self, color: bool, depth: bool, stencil: bool) -> &mut Self {
        let mut bits = 0;

        if color {
            bits = bits | gl::COLOR_BUFFER_BIT;
        }
        if depth {
            bits = bits | gl::DEPTH_BUFFER_BIT;
        }
        if stencil {
            bits = bits | gl::STENCIL_BUFFER_BIT;
        }

        self.clear_bits(bits)
    }

    #[inline(always)]
    pub fn clear_bits(&mut self, bits: GLenum) -> &mut Self {
        unsafe { gl::Clear(bits); }
        self
    }

    pub fn force(&self) -> bool {
        self.program_force
    }

    pub fn enable_attribute(&mut self, index: usize) -> bool {
        let ref mut value = self.enabled_attributes[index];

        if !*value {
            unsafe { gl::EnableVertexAttribArray(index as u32); }
            *value = true;
            true
        } else {
            false
        }
    }
    pub fn disable_attribute(&mut self, index: usize) -> bool {
        let ref mut value = self.enabled_attributes[index];

        if *value {
            unsafe { gl::DisableVertexAttribArray(index as u32); }
            *value = false;
            true
        } else {
            false
        }
    }

    fn disable_attributes(&mut self) {
        let mut index = 0u32;
        let ref mut enabled_attributes = self.enabled_attributes;

        for value in enabled_attributes {
            if *value {
                unsafe { gl::DisableVertexAttribArray(index); }
                *value = false;
            }
            index += 1;
        }
    }

    pub fn set_buffer(&mut self, buffer: &Buffer) -> bool {
        match buffer.kind() {
            gl::ARRAY_BUFFER => self.set_array_buffer(buffer),
            gl::ELEMENT_ARRAY_BUFFER => self.set_element_array_buffer(buffer),
            _ => false,
        }
    }
    pub fn remove_buffer(&mut self) -> bool {
        if self.current_array_buffer != 0 || self.current_element_array_buffer != 0 {
            self.disable_attributes();
            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            }
            self.current_array_buffer = 0;
            self.current_element_array_buffer = 0;
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn set_array_buffer(&mut self, buffer: &Buffer) -> bool {
        let id = buffer.id();

        if self.current_array_buffer != id {
            self.disable_attributes();
            unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, id as GLuint); }
            self.current_array_buffer = id;
            true
        } else {
            false
        }
    }
    #[inline(always)]
    fn set_element_array_buffer(&mut self, buffer: &Buffer) -> bool {
        let id = buffer.id();

        if self.current_element_array_buffer != id {
            self.disable_attributes();
            unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id as GLuint); }
            self.current_element_array_buffer = id;
            true
        } else {
            false
        }
    }

    pub fn set_attrib_pointer(
        &mut self, location: usize, item_size: usize, kind: GLenum, stride: usize, offset: usize, force: bool
    ) -> bool {
        if self.enable_attribute(location as usize) || force {
            unsafe {
                gl::VertexAttribPointer(
                    location as GLuint,
                    item_size as GLint,
                    kind,
                    gl::FALSE,
                    stride as GLsizei,
                    offset as *const i32 as *const _
                );
            }
            true
        } else {
            false
        }
    }

    pub fn set_vertex_array(&mut self, vertex_array: &VertexArray) -> bool {
        let id = vertex_array.id();

        if self.current_vertex_array != id {
            unsafe { gl::BindVertexArray(id as GLuint); }
            self.current_vertex_array = id;
            true
        } else {
            false
        }
    }
    pub fn remove_vertex_array(&mut self) -> bool {
        if self.current_vertex_array != 0 {
            unsafe { gl::BindVertexArray(0); }
            self.current_vertex_array = 0;
            true
        } else {
            false
        }
    }

    pub fn set_program(&mut self, program: &Program, force: bool) -> bool {
        let id = program.id();

        if self.program != id || force {
            self.program = id;
            self.program_force = true;
            unsafe { gl::UseProgram(id as GLuint); }
        } else {
            if self.texture_index != 0 || self.active_index != -1 {
                self.program_force = true;
            } else {
                self.program_force = false;
            }
        }

        self.texture_index = 0;
        self.active_index = -1;

        true
    }

    pub fn set_texture(&mut self, texture: Texture, force: bool) -> bool {
        true
    }

    pub fn new_program(&self) -> Program {
        Program::new()
    }

    pub fn new_buffer(&self) -> Buffer {
        Buffer::new()
    }

    pub fn new_vertex_array(&self) -> VertexArray {
        VertexArray::new()
    }

    pub fn has_extenstion(&self, string: String) -> bool {
        match self.extenstions.iter().position(|e| *e == string) {
            Some(index) => true,
            None => false,
        }
    }

    fn get_gl_info(&mut self) {
        let mut vs_high_float_precision = 0i32;
        let mut vs_high_float_range = 0i32;
        unsafe { gl::GetShaderPrecisionFormat(gl::VERTEX_SHADER, gl::HIGH_FLOAT, &mut vs_high_float_range, &mut vs_high_float_precision); }
        let mut vs_mediump_float_precision = 0i32;
        let mut vs_mediump_float_range = 0i32;
        unsafe { gl::GetShaderPrecisionFormat(gl::VERTEX_SHADER, gl::MEDIUM_FLOAT, &mut vs_mediump_float_range, &mut vs_mediump_float_precision); }

        let mut fs_high_float_precision = 0i32;
        let mut fs_high_float_range = 0i32;
        unsafe { gl::GetShaderPrecisionFormat(gl::FRAGMENT_SHADER, gl::HIGH_FLOAT, &mut fs_high_float_range, &mut fs_high_float_precision); }
        let mut fs_mediump_float_precision = 0i32;
        let mut fs_mediump_float_range = 0i32;
        unsafe { gl::GetShaderPrecisionFormat(gl::FRAGMENT_SHADER, gl::MEDIUM_FLOAT, &mut fs_mediump_float_range, &mut fs_mediump_float_precision); }

        let highp_available = vs_high_float_precision as i32 > 0i32 && fs_high_float_precision as i32 > 0i32;
        let mediump_available = vs_mediump_float_precision as i32 > 0i32 && fs_mediump_float_precision as i32 > 0i32;

        self.precision = if !highp_available {
            if mediump_available {
                MEDIUMP
            } else {
                LOWP
            }
        } else {
            HIGHP
        };

        unsafe {
            let ptr = gl::GetString(gl::VERSION);
            string_from_ptr(ptr as usize, &mut self.version_string);

            let cap = Regex::new(r"(\d+).(\d+).(\d+)").unwrap().captures(self.version_string.as_str()).unwrap();

            let mut major = cap.at(1).unwrap_or("3").parse::<i32>().unwrap();
            let mut minor = cap.at(2).unwrap_or("1").parse::<i32>().unwrap();

            if major > 2 {
                gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
                self.major = major as usize;
                gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
                self.minor = minor as usize;
            }

            parse_extenstions(&mut self.extenstions, self.major);

            let mut max_textures = 0;
            gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut max_textures);
            self.max_textures = max_textures as usize;

            let mut max_vertex_textures = 0;
            gl::GetIntegerv(gl::MAX_VERTEX_TEXTURE_IMAGE_UNITS, &mut max_vertex_textures);
            self.max_vertex_textures = max_vertex_textures as usize;

            let mut max_texture_size = 0;
            gl::GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut max_texture_size);
            self.max_texture_size = max_texture_size as usize;

            let mut max_cube_texture_size = 0;
            gl::GetIntegerv(gl::MAX_CUBE_MAP_TEXTURE_SIZE, &mut max_cube_texture_size);
            self.max_cube_texture_size = max_cube_texture_size as usize;

            let mut max_render_buffer_size = 0;
            gl::GetIntegerv(gl::MAX_RENDERBUFFER_SIZE, &mut max_render_buffer_size);
            self.max_render_buffer_size = max_render_buffer_size as usize;

            let mut vs_max_uniforms = 0;
            let mut fs_max_uniforms = 0;
            gl::GetIntegerv(gl::MAX_VERTEX_UNIFORM_VECTORS, &mut vs_max_uniforms);
            gl::GetIntegerv(gl::MAX_FRAGMENT_UNIFORM_VECTORS, &mut fs_max_uniforms);
            self.max_uniforms = if vs_max_uniforms < fs_max_uniforms { vs_max_uniforms } else { fs_max_uniforms } as usize * 4;

            let mut max_varyings = 0;
            gl::GetIntegerv(gl::MAX_VARYING_VECTORS, &mut max_varyings);
            self.max_varyings = max_varyings as usize * 4;

            let mut max_attributes = 0;
            gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_attributes);
            self.max_attributes = max_attributes as usize;

            for _ in 0..self.max_attributes {
                self.enabled_attributes.push(false);
            }
        }
    }
}

unsafe fn string_from_ptr(ptr: usize, string: &mut String) {
    let mut i = ptr as usize;
    loop {
        let ch = *(i as *const u8);
        if ch != 0 {
            string.push(ch as char);
            i = i + 1;
        } else {
            break;
        }
    }
}

unsafe fn parse_extenstions(extenstions: &mut Vec<String>, major_version: usize) {
    if major_version > 2 {
        let mut count = 0;
        gl::GetIntegerv(gl::NUM_EXTENSIONS, &mut count);

        for i in 0..(count as u32) {
            let mut string = String::new();
            string_from_ptr(gl::GetStringi(gl::EXTENSIONS, i) as usize, &mut string);
            extenstions.push(string);
        }
    } else {
        let mut string = String::new();
        string_from_ptr(gl::GetString(gl::EXTENSIONS) as usize, &mut string);

        for extenstion in string.split_whitespace() {
            extenstions.push(String::from(extenstion));
        }
    }
}
