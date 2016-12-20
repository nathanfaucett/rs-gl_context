use collections::vec::Vec;
use collections::string::String;

use core::mem;

use gl;
use gl::types::*;
use regex::Regex;

use enums::*;
use buffer::Buffer;
use program::Program;
use texture::Texture;
use vertex_array::VertexArray;
use framebuffer::Framebuffer;
use renderbuffer::Renderbuffer;


static HIGHP: &'static str = "highp";
static MEDIUMP: &'static str = "mediump";
static LOWP: &'static str = "lowp";


#[derive(Debug)]
pub struct Context {
    version: String,

    major: usize,
    minor: usize,
    glsl_major: usize,
    glsl_minor: usize,

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

    precision: &'static str,

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

    current_array_buffer: GLuint,
    current_element_array_buffer: GLuint,

    current_vertex_array: GLuint,
    current_framebuffer: GLuint,
    current_renderbuffer: GLuint,

    program: GLuint,
    program_force: bool,

    texture_index: GLuint,
    current_texture_index: GLint,
    current_texture: GLuint,
}

impl Context {

    pub fn new() -> Self {
        Context {
            version: String::new(),

            major: 0,
            minor: 0,
            glsl_major: 0,
            glsl_minor: 0,

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

            precision: HIGHP,

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
            depth_write: true,
            line_width: 1f32,

            current_array_buffer: 0,
            current_element_array_buffer: 0,

            current_vertex_array: 0,
            current_framebuffer: 0,
            current_renderbuffer: 0,

            program: 0,
            program_force: false,

            texture_index: 0,
            current_texture_index: -1,
            current_texture: 0,
        }
    }

    pub fn get_version(&self) -> String { self.version.clone() }

    pub fn get_major(&self) -> usize { self.major }
    pub fn get_minor(&self) -> usize { self.minor }
    pub fn get_glsl_major(&self) -> usize { self.glsl_major }
    pub fn get_glsl_minor(&self) -> usize { self.glsl_minor }

    pub fn get_max_anisotropy(&self) -> usize { self.max_anisotropy }
    pub fn get_max_textures(&self) -> usize { self.max_textures }
    pub fn get_max_vertex_textures(&self) -> usize { self.max_vertex_textures }
    pub fn get_max_texture_size(&self) -> usize { self.max_texture_size }
    pub fn get_get_max_cube_texture_size(&self) -> usize { self.max_cube_texture_size }
    pub fn get_max_render_buffer_size(&self) -> usize { self.max_render_buffer_size }

    pub fn get_max_uniforms(&self) -> usize { self.max_uniforms }
    pub fn get_max_varyings(&self) -> usize { self.max_varyings }
    pub fn get_max_attributes(&self) -> usize { self.max_attributes }

    pub fn get_precision(&self) -> &'static str { self.precision }
    pub fn get_force(&self) -> bool { self.program_force }

    pub fn init(&mut self) -> &mut Self {
        self.reset()
    }

    pub fn reset(&mut self) -> &mut Self {

        self.version.clear();

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

        self.precision = HIGHP;

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
        self.depth_write = true;
        self.line_width = 1f32;

        self.current_array_buffer = 0;
        self.current_element_array_buffer = 0;

        self.current_vertex_array = 0;
        self.current_framebuffer = 0;
        self.current_renderbuffer = 0;

        self.program = 0;
        self.program_force = false;

        self.texture_index = 0;
        self.current_texture_index = -1;
        self.current_texture = 0;

        self.get_gl_info();
        self.gl_reset();

        self
    }

    pub fn soft_reset(&mut self) -> &mut Self {

        self.clear_color[0] = 0f32;
        self.clear_color[1] = 0f32;
        self.clear_color[2] = 0f32;
        self.clear_color[3] = 1f32;

        self.viewport_x = 0;
        self.viewport_y = 0;
        self.viewport_width = 1;
        self.viewport_height = 1;

        self
    }

    fn gl_reset(&mut self) -> &mut Self {
        unsafe {
            gl::FrontFace(gl::CCW);
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }

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

        self.set_clear_color_unchecked(&self.clear_color);
        self.clear(true, true, true);

        self
    }

    #[inline(always)]
    pub fn set_viewport_unchecked(&self, x: usize, y: usize, width: usize, height: usize) -> &Self {
        unsafe { gl::Viewport(x as GLint, y as GLint, width as GLsizei, height as GLsizei); }
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
        unsafe { gl::DepthMask(if depth_write {gl::TRUE} else {gl::FALSE}); }
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
        unsafe { gl::LineWidth(line_width as GLfloat); }
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
    fn enable_blending(&mut self) {
        if self.blending_disabled {
            unsafe { gl::Enable(gl::BLEND); }
            self.blending_disabled = false;
        }
    }
    #[inline(always)]
    pub fn set_blending_unchecked(&mut self, blending: Blending) -> &mut Self {
        match blending {
            Blending::Additive => {
                self.enable_blending();
                unsafe {
                    gl::BlendEquation(gl::FUNC_ADD);
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
                }
            },
            Blending::Subtractive => {
                self.enable_blending();
                unsafe {
                    gl::BlendEquation(gl::FUNC_ADD);
                    gl::BlendFunc(gl::ZERO, gl::ONE_MINUS_SRC_COLOR);
                }
            },
            Blending::Multiply => {
                self.enable_blending();
                unsafe {
                    gl::BlendEquation(gl::FUNC_ADD);
                    gl::BlendFunc(gl::ZERO, gl::SRC_COLOR);
                }
            },
            Blending::Default => {
                self.enable_blending();
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
    fn enable_cull_face(&mut self) {
        if self.cull_face_disabled {
            unsafe { gl::Enable(gl::CULL_FACE); }
            self.cull_face_disabled = false;
        }
    }
    #[inline(always)]
    pub fn set_cull_face_unchecked(&mut self, cull_face: CullFace) -> &mut Self {
        match cull_face {
            CullFace::Back => {
                self.enable_cull_face();
                unsafe { gl::CullFace(gl::BACK); }
            },
            CullFace::Front => {
                self.enable_cull_face();
                unsafe { gl::CullFace(gl::FRONT); }
            },
            CullFace::FrontAndBack => {
                self.enable_cull_face();
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
    fn enable_depth_test(&mut self) {
        if self.depth_test_disabled {
            unsafe { gl::Enable(gl::DEPTH_TEST); }
            self.depth_test_disabled = false;
        }
    }
    #[inline(always)]
    pub fn set_depth_func_unchecked(&mut self, depth_func: Depth) -> &mut Self {
        match depth_func {
            Depth::Never => {
                self.enable_depth_test();
                unsafe { gl::DepthFunc(gl::NEVER); }
            },
            Depth::LessThan => {
                self.enable_depth_test();
                unsafe { gl::DepthFunc(gl::LESS); }
            },
            Depth::Equal => {
                self.enable_depth_test();
                unsafe { gl::DepthFunc(gl::EQUAL); }
            },
            Depth::LessThanOrEqual => {
                self.enable_depth_test();
                unsafe { gl::DepthFunc(gl::LEQUAL); }
            },
            Depth::GreaterThan => {
                self.enable_depth_test();
                unsafe { gl::DepthFunc(gl::GREATER); }
            },
            Depth::NotEqual => {
                self.enable_depth_test();
                unsafe { gl::DepthFunc(gl::NOTEQUAL); }
            },
            Depth::GreaterThanOrEqual => {
                self.enable_depth_test();
                unsafe { gl::DepthFunc(gl::GEQUAL); }
            },
            Depth::Always => {
                self.enable_depth_test();
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
    pub fn set_clear_color_unchecked<'a>(&self, color: &'a [f32; 4]) -> &Self {
        unsafe { gl::ClearColor(color[0], color[1], color[2], color[3]); }
        self
    }
    pub fn set_clear_color<'a>(&mut self, color: &'a [f32; 4]) -> &mut Self {
        if &self.clear_color != color {
            self.clear_color[0] = color[0];
            self.clear_color[1] = color[1];
            self.clear_color[2] = color[2];
            self.clear_color[3] = color[3];
            self.set_clear_color_unchecked(color);
        }
        self
    }

    pub fn clear(&mut self, color: bool, depth: bool, stencil: bool) -> &mut Self {
        let mut bits: GLbitfield = 0;

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

    pub fn enable_attribute(&mut self, index: usize, force: bool) -> bool {
        let ref mut value = self.enabled_attributes[index];

        if force || !*value {
            unsafe { gl::EnableVertexAttribArray(index as GLuint); }
            *value = true;
            true
        } else {
            false
        }
    }
    pub fn disable_attribute(&mut self, index: usize) -> bool {
        let ref mut value = self.enabled_attributes[index];

        if *value {
            unsafe { gl::DisableVertexAttribArray(index as GLuint); }
            *value = false;
            true
        } else {
            false
        }
    }

    fn disable_attributes(&mut self) {
        let mut index: GLuint = 0;
        let ref mut enabled_attributes = self.enabled_attributes;

        for value in enabled_attributes {
            if *value {
                unsafe { gl::DisableVertexAttribArray(index); }
                *value = false;
            }
            index += 1;
        }
    }

    pub fn set_buffer(&mut self, buffer: &Buffer, force: bool) -> bool {
        match buffer.get_kind() {
            gl::ARRAY_BUFFER => self.set_array_buffer(buffer, force),
            gl::ELEMENT_ARRAY_BUFFER => self.set_element_array_buffer(buffer, force),
            _ => false,
        }
    }
    pub fn remove_buffer(&mut self, force: bool) -> bool {
        if force || self.current_array_buffer != 0 || self.current_element_array_buffer != 0 {
            self.disable_attributes();
            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, 0 as GLuint);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0 as GLuint);
            }
            self.current_array_buffer = 0;
            self.current_element_array_buffer = 0;
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn set_array_buffer(&mut self, buffer: &Buffer, force: bool) -> bool {
        let id = buffer.get_id();

        if force || self.current_array_buffer != id {
            self.disable_attributes();
            unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, id); }
            self.current_array_buffer = id;
            true
        } else {
            false
        }
    }
    #[inline(always)]
    fn set_element_array_buffer(&mut self, buffer: &Buffer, force: bool) -> bool {
        let id = buffer.get_id();

        if force || self.current_element_array_buffer != id {
            self.disable_attributes();
            unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id); }
            self.current_element_array_buffer = id;
            true
        } else {
            false
        }
    }

    pub fn set_attrib_pointer(
        &mut self, location: GLuint, item_size: GLint, kind: GLenum, stride: GLsizei, offset: GLint, force: bool
    ) -> bool {
        if self.enable_attribute(location as usize, force) {
            unsafe {
                gl::VertexAttribPointer(
                    location,
                    item_size,
                    kind,
                    gl::FALSE,
                    stride,
                    mem::transmute(offset as usize)
                );
            }
            true
        } else {
            false
        }
    }

    pub fn set_vertex_array(&mut self, vertex_array: &VertexArray, force: bool) -> bool {
        let id = vertex_array.get_id();

        if force || self.current_vertex_array != id {
            unsafe { gl::BindVertexArray(id); }
            self.current_vertex_array = id;
            true
        } else {
            false
        }
    }
    pub fn remove_vertex_array(&mut self, force: bool) -> bool {
        if force || self.current_vertex_array != 0 {
            unsafe { gl::BindVertexArray(0); }
            self.current_vertex_array = 0;
            true
        } else {
            false
        }
    }

    pub fn set_framebuffer(&mut self, framebuffer: &Framebuffer, force: bool) -> bool {
        let id = framebuffer.get_id();

        if force || self.current_framebuffer != id {
            unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, id); }
            self.current_framebuffer = id;
            self.soft_reset();
            true
        } else {
            false
        }
    }
    pub fn remove_framebuffer(&mut self, force: bool) -> bool {
        if force || self.current_framebuffer != 0 {
            unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, 0); }
            self.current_framebuffer = 0;
            self.soft_reset();
            true
        } else {
            false
        }
    }

    pub fn set_renderbuffer(&mut self, renderbuffer: &Renderbuffer, force: bool) -> bool {
        let id = renderbuffer.get_id();

        if force || self.current_renderbuffer != id {
            unsafe { gl::BindRenderbuffer(gl::RENDERBUFFER, id); }
            self.current_renderbuffer = id;
            true
        } else {
            false
        }
    }
    pub fn remove_renderbuffer(&mut self, force: bool) -> bool {
        if force || self.current_renderbuffer != 0 {
            unsafe { gl::BindRenderbuffer(gl::RENDERBUFFER, 0); }
            self.current_renderbuffer = 0;
            true
        } else {
            false
        }
    }

    pub fn set_texture(&mut self, location: GLint, texture: &Texture, force: bool) -> bool {
        let id = texture.get_id();
        let index = self.texture_index;
        let current_texture_index = self.current_texture_index;

        self.texture_index = index + 1;
        self.current_texture_index = index as GLint;

        if force || self.current_texture != id {
            let needs_update = force || self.program_force || current_texture_index != index as GLint;

            if needs_update {
                unsafe { gl::ActiveTexture(gl::TEXTURE0 + index); }
                unsafe { gl::Uniform1i(location, index as GLint); }
            }
            unsafe { gl::BindTexture(gl::TEXTURE_2D, id); }

            self.current_texture = id;

            true
        } else {
            false
        }
    }
    pub fn remove_texture(&mut self, force: bool) -> bool {
        if force || self.current_texture != 0 {
            self.texture_index = 0;
            self.current_texture_index = -1;
            unsafe { gl::BindTexture(gl::TEXTURE_2D, 0 as GLuint); }
            true
        } else {
            false
        }
    }

    pub fn set_program(&mut self, program: &Program, force: bool) -> bool {
        let id = program.get_id();

        if force || self.program != id {
            self.program = id;
            self.program_force = true;
            unsafe { gl::UseProgram(id); }
        } else {
            if self.texture_index != 0 || self.current_texture_index != -1 {
                self.program_force = true;
            } else {
                self.program_force = false;
            }
        }

        self.texture_index = 0;
        self.current_texture_index = -1;

        true
    }
    pub fn remove_program(&mut self, force: bool) -> bool {
        if force || self.program != 0 {
            self.program = 0;
            self.program_force = true;
            unsafe { gl::UseProgram(0 as GLuint); }
        } else {
            if self.texture_index != 0 || self.current_texture_index != -1 {
                self.program_force = true;
            } else {
                self.program_force = false;
            }
        }

        self.texture_index = 0;
        self.current_texture_index = -1;

        true
    }

    pub fn new_buffer(&self) -> Buffer {
        Buffer::new()
    }
    pub fn new_framebuffer(&self) -> Framebuffer {
        Framebuffer::new()
    }
    pub fn new_program(&self) -> Program {
        Program::new()
    }
    pub fn new_renderbuffer(&self) -> Renderbuffer {
        Renderbuffer::new()
    }
    pub fn new_texture(&self) -> Texture {
        Texture::new()
    }
    pub fn new_vertex_array(&self) -> VertexArray {
        VertexArray::new()
    }

    pub fn has_extenstion(&self, string: String) -> bool {
        match self.extenstions.iter().position(|e| *e == string) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_error(&self) -> GLenum {
        unsafe { gl::GetError() }
    }

    fn get_gl_info(&mut self) {
        let mut vs_high_float_precision: GLint = 0;
        let mut vs_high_float_range: GLint = 0;
        unsafe {
            gl::GetShaderPrecisionFormat(
                gl::VERTEX_SHADER,
                gl::HIGH_FLOAT,
                &mut vs_high_float_range,
                &mut vs_high_float_precision
            );
        }

        let mut vs_mediump_float_precision: GLint = 0;
        let mut vs_mediump_float_range: GLint = 0;
        unsafe {
            gl::GetShaderPrecisionFormat(
                gl::VERTEX_SHADER,
                gl::MEDIUM_FLOAT,
                &mut vs_mediump_float_range,
                &mut vs_mediump_float_precision
            );
        }

        let mut fs_high_float_precision: GLint = 0;
        let mut fs_high_float_range: GLint = 0;
        unsafe {
            gl::GetShaderPrecisionFormat(
                gl::FRAGMENT_SHADER,
                gl::HIGH_FLOAT,
                &mut fs_high_float_range,
                &mut fs_high_float_precision
            );
        }

        let mut fs_mediump_float_precision: GLint = 0;
        let mut fs_mediump_float_range: GLint = 0;
        unsafe {
            gl::GetShaderPrecisionFormat(
                gl::FRAGMENT_SHADER,
                gl::MEDIUM_FLOAT,
                &mut fs_mediump_float_range,
                &mut fs_mediump_float_precision
            );
        }

        let highp_available = vs_high_float_precision > 0 && fs_high_float_precision > 0;
        let mediump_available = vs_mediump_float_precision > 0 && fs_mediump_float_precision > 0;

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
            string_from_ptr(ptr, &mut self.version);

            let cap = Regex::new(r"(\d+).(\d+).(\d+)").unwrap().captures(self.version.as_str()).unwrap();
            let mut major = cap.at(1).unwrap_or("3").parse::<i32>().unwrap();
            let mut minor = cap.at(2).unwrap_or("1").parse::<i32>().unwrap();

            if major > 2 {
                gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
                self.major = major as usize;
                gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
                self.minor = minor as usize;
            }

            get_glsl_version(self.major, self.minor, &mut self.glsl_major, &mut self.glsl_minor);
            parse_extenstions(&mut self.extenstions, self.major);
        }

        unsafe {
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
            self.max_uniforms = if vs_max_uniforms < fs_max_uniforms {
                vs_max_uniforms
            } else {
                fs_max_uniforms
            } as usize * 4;

            let mut max_varyings = 0;
            gl::GetIntegerv(gl::MAX_VARYING_VECTORS, &mut max_varyings);
            self.max_varyings = max_varyings as usize * 4;

            let mut max_attributes = 0;
            gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_attributes);
            self.max_attributes = max_attributes as usize;
        }

        for _ in 0..self.max_attributes {
            self.enabled_attributes.push(false);
        }
    }
}

unsafe fn string_from_ptr(ptr: *const u8, string: &mut String) {
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
            string_from_ptr(gl::GetStringi(gl::EXTENSIONS, i), &mut string);
            extenstions.push(string);
        }
    } else {
        let mut string = String::new();
        string_from_ptr(gl::GetString(gl::EXTENSIONS), &mut string);

        for extenstion in string.split_whitespace() {
            extenstions.push(String::from(extenstion));
        }
    }
}

fn get_glsl_version(major: usize, minor: usize, glsl_major: &mut usize, glsl_minor: &mut usize) {
    if major <= 3 && minor <= 2 {
        *glsl_major = 1;
        *glsl_minor = if major == 3 && minor == 2 {
            5
        } else if major == 3 && minor == 1 {
            4
        } else if major == 3 && minor == 0 {
            3
        } else if major == 2 && minor == 1 {
            2
        } else {
            1
        }
    } else {
        *glsl_major = major;
        *glsl_minor = minor;
    }
}
