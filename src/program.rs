use core::ptr;
use core::ops::Drop;
use core::any::Any;
use collections::vec::Vec;
use collections::boxed::Box;
use collections::str;
use collections::string::String;
use collections::btree_map::BTreeMap;

use gl;
use gl::types::*;

use uniform::{new_uniform, Uniform};
use attribute::{new_attribute, Attribute};
use buffer::Buffer;
use context::Context;


#[derive(Debug)]
pub struct Program {
    id: usize,
    uniforms: BTreeMap<String, Box<Uniform>>,
    attributes: BTreeMap<String, Box<Attribute>>,
}

impl Drop for Program {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe { gl::DeleteProgram(self.id as GLuint); }
        }
    }
}

impl Program {

    pub fn new() -> Self {
        Program {
            id: 0,
            uniforms: BTreeMap::new(),
            attributes: BTreeMap::new(),
        }
    }

    pub fn id(&self) -> usize { self.id }

    pub fn set_uniform(&mut self, name: String, context: &Context, value: &Any, force: bool) -> bool {
        match self.uniforms.get_mut(&name) {
            Some(ref mut uniform) => uniform.set(context, value, force),
            None => panic!("No uniform named {:?} found", name),
        }
    }
    pub fn set_uniform_unchecked(&mut self, name: String, context: &Context, value: &Any, force: bool) -> bool {
        match self.uniforms.get_mut(&name) {
            Some(ref mut uniform) => uniform.set_unchecked(context, value, force),
            None => panic!("No uniform named {:?} found", name),
        }
    }
    pub fn set_attribute(&mut self, name: String, context: &mut Context, buffer: &Buffer, offset: usize, force: bool) -> bool {
        match self.attributes.get(&name) {
            Some(ref attribute) => attribute.set(context, buffer, offset, force),
            None => panic!("No attribute named {:?} found", name),
        }
    }

    pub fn set(&mut self, vertex: String, fragment: String) -> &mut Self {
        {
            let ref mut uniforms = self.uniforms;
            let ref mut attributes = self.attributes;

            if self.id != 0 {
                uniforms.clear();
                attributes.clear();
                unsafe { gl::DeleteProgram(self.id as GLuint); }
            }

            let id = create_program(vertex, fragment);
            self.id = id as usize;
            parse_uniforms(id, uniforms);
            parse_attributes(id, attributes);
        }
        self
    }
}

fn parse_uniforms(program: GLuint, uniforms: &mut BTreeMap<String, Box<Uniform>>) {
    let mut max_length = 0;
    let mut active_length = 0;
    unsafe {
        gl::GetProgramiv(program, gl::ACTIVE_UNIFORM_MAX_LENGTH, &mut max_length);
        gl::GetProgramiv(program, gl::ACTIVE_UNIFORMS, &mut active_length);
    }

    for i in 0..active_length {
        let mut length = 0;
        let mut size = 0;
        let mut kind = 0;

        let mut buf = Vec::with_capacity(max_length as usize);
        let buf_ptr = buf.as_mut_ptr() as *mut GLchar;
        let location;

        unsafe {
            gl::GetActiveUniform(program, i as u32, max_length, &mut length, &mut size, &mut kind, buf_ptr);
            buf.set_len(length as usize);
            location = gl::GetUniformLocation(program, buf_ptr);
        }

        let mut name = match String::from_utf8(buf) {
            Ok(string) => string,
            Err(vec) => panic!("Could not convert uniform name from buffer: {:?}", vec)
        };

        if name.chars().nth(name.len() - 1).expect("Unexpected empty uniform name") == ']' {
            name.pop();
            name.pop();
            name.pop();
        }

        uniforms.insert(name.clone(), new_uniform(name, kind, size as usize, location as usize));
    }
}

fn parse_attributes(program: GLuint, attributes: &mut BTreeMap<String, Box<Attribute>>) {
    let mut max_length = 0;
    let mut active_length = 0;
    unsafe {
        gl::GetProgramiv(program, gl::ACTIVE_ATTRIBUTE_MAX_LENGTH, &mut max_length);
        gl::GetProgramiv(program, gl::ACTIVE_ATTRIBUTES, &mut active_length);
    }

    for i in 0..active_length {
        let mut length = 0;
        let mut size = 0;
        let mut kind = 0;

        let mut buf = Vec::with_capacity(max_length as usize);
        let buf_ptr = buf.as_mut_ptr() as *mut GLchar;
        let location;

        unsafe {
            gl::GetActiveAttrib(program, i as u32, max_length, &mut length, &mut size, &mut kind, buf_ptr);
            buf.set_len(length as usize);
            location = gl::GetAttribLocation(program, buf_ptr);
        }

        let name = match String::from_utf8(buf) {
            Ok(string) => string,
            Err(vec) => panic!("Could not convert attribute name from buffer: {:?}", vec)
        };

        attributes.insert(name.clone(), new_attribute(name, kind, size as usize, location as usize));
    }
}

fn create_program(vertex: String, fragment: String) -> GLuint {
    let program = unsafe { gl::CreateProgram() };

    let vertex_shader = create_shader(vertex, gl::VERTEX_SHADER);
    unsafe {
        gl::AttachShader(program, vertex_shader);
        gl::DeleteShader(vertex_shader);
    }

    let fragment_shader = create_shader(fragment, gl::FRAGMENT_SHADER);
    unsafe {
        gl::AttachShader(program, fragment_shader);
        gl::DeleteShader(fragment_shader);
    }

    unsafe {
        gl::LinkProgram(program);
        gl::ValidateProgram(program);
        gl::UseProgram(program);
    }

    let mut status = 0;
    unsafe { gl::GetProgramiv(program, gl::LINK_STATUS, &mut status) };
    if status == 0 {
        let mut len = 0;
        unsafe { gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len) };

        let mut buf = Vec::with_capacity(len as usize);
        let buf_ptr = buf.as_mut_ptr() as *mut GLchar;
        unsafe {
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf_ptr);
            buf.set_len(len as usize);
        };

        match String::from_utf8(buf) {
            Ok(log) => panic!("{:?}", log),
            Err(vec) => panic!("Could not convert link log from buffer: {:?}", vec)
        }
    }

    program
}

fn create_shader(source: String, kind: GLenum) -> GLuint {
    let shader = unsafe { gl::CreateShader(kind) };

    let sources = &[source.as_bytes()];
    let lengths = &[source.len() as i32];

    unsafe {
        gl::ShaderSource(shader, 1, (sources as *const _) as *const *const GLchar, lengths as *const _);
        gl::CompileShader(shader);
    }

    let mut status = 0;
    unsafe { gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status) };
    if status == 0 {
        let mut len = 0;
        unsafe { gl::GetProgramiv(shader, gl::INFO_LOG_LENGTH, &mut len) };

        let mut buf = Vec::with_capacity(len as usize);
        let buf_ptr = buf.as_mut_ptr() as *mut GLchar;
        unsafe {
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf_ptr);
            buf.set_len(len as usize);
        };

        match String::from_utf8(buf) {
            Ok(log) => panic!("{:?}", log),
            Err(vec) => panic!("Could not convert link log from buffer: {:?}", vec)
        }
    }

    shader
}
