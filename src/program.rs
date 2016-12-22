use alloc::boxed::Box;
use collections::vec::Vec;
use collections::str;
use collections::string::String;

use core::ptr;
use core::mem;
use core::ops::Drop;
use core::any::Any;

use gl;
use gl::types::*;

use hash_map::HashMap;
use insert::Insert;
use map::Map;

use uniform::{new_uniform, Uniform};
use attribute::{new_attribute, Attribute};
use buffer::Buffer;
use context::Context;


#[derive(Debug)]
pub struct Program {
    id: GLuint,
    uniforms: HashMap<String, Box<Uniform>>,
    attributes: HashMap<String, Box<Attribute>>,
}

impl Drop for Program {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe { gl::DeleteProgram(self.id); }
        }
    }
}

impl Program {

    pub fn new() -> Self {
        Program {
            id: 0,
            uniforms: HashMap::new(),
            attributes: HashMap::new(),
        }
    }

    pub fn get_id(&self) -> GLuint { self.id }

    pub fn has_uniform(&self, name: &str) -> bool {self.uniforms.contains_key(&String::from(name))}
    pub fn get_uniforms(&self) -> &HashMap<String, Box<Uniform>> {&self.uniforms}
    pub fn get_uniforms_mut(&mut self) -> &mut HashMap<String, Box<Uniform>> {&mut self.uniforms}

    pub fn set_uniform(&mut self, name: &str, context: &mut Context, value: &Any, force: bool) -> bool {
        match self.uniforms.get_mut(name) {
            Some(ref mut uniform) => uniform.set(context, value, force),
            None => panic!("No uniform named {:?} found", name),
        }
    }
    pub fn set_uniform_unchecked(&mut self, name: &str, context: &mut Context, value: &Any, force: bool) -> bool {
        match self.uniforms.get_mut(name) {
            Some(ref mut uniform) => uniform.set_unchecked(context, value, force),
            None => panic!("No uniform named {:?} found", name),
        }
    }

    pub fn has_attribute(&self, name: &str) -> bool {self.attributes.contains_key(&String::from(name))}
    pub fn get_attributes(&self) -> &HashMap<String, Box<Attribute>> {&self.attributes}
    pub fn get_attributes_mut(&mut self) -> &mut HashMap<String, Box<Attribute>> {&mut self.attributes}

    pub fn set_attribute(&mut self, name: &str, context: &mut Context, buffer: &Buffer, offset: usize, force: bool) -> bool {
        match self.attributes.get(name) {
            Some(ref attribute) => attribute.set(context, buffer, offset, force),
            None => panic!("No attribute named {:?} found", name),
        }
    }

    pub fn set(&mut self, vertex: &str, fragment: &str) -> &mut Self {
        let vs = compile_shader(vertex, gl::VERTEX_SHADER);
        let fs = compile_shader(fragment, gl::FRAGMENT_SHADER);
        let id = link_program(vs, fs);
        self.set_program_id(id)
    }

    pub fn set_program_id(&mut self, id: GLuint) -> &mut Self {
        {
            let ref mut uniforms = self.uniforms;
            let ref mut attributes = self.attributes;

            if self.id != 0 {
                uniforms.clear();
                attributes.clear();
                unsafe { gl::DeleteProgram(self.id); }
            }

            self.id = id;
            parse_uniforms(id, uniforms);
            parse_attributes(id, attributes);
            unsafe { gl::UseProgram(0) };
        }
        self
    }
}

fn parse_uniforms(program: GLuint, uniforms: &mut HashMap<String, Box<Uniform>>) {
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

        uniforms.insert(name.clone(), new_uniform(name, kind, size as usize, location));
    }
}

fn parse_attributes(program: GLuint, attributes: &mut HashMap<String, Box<Attribute>>) {
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

        attributes.insert(name.clone(), new_attribute(name.clone(), kind, size as usize, location));
    }
}

pub fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    let program = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(program, vertex_shader);
        gl::DeleteShader(vertex_shader);

        gl::AttachShader(program, fragment_shader);
        gl::DeleteShader(fragment_shader);

        gl::LinkProgram(program);
        gl::ValidateProgram(program);
        gl::UseProgram(program);
    }

    let mut status = 0;
    unsafe { gl::GetProgramiv(program, gl::LINK_STATUS, &mut status) };
    if status != (gl::TRUE as GLint) {
        let mut len: GLint = 0;
        unsafe { gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len); }
        let mut buf = Vec::with_capacity(len as usize);
        unsafe {
            buf.set_len(len as usize);
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
        }
        panic!("{}", str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));
    }

    program
}

pub fn compile_shader(source: &str, kind: GLenum) -> GLuint {
    let shader = unsafe { gl::CreateShader(kind) };

    unsafe {
        let ptr: *const u8 = source.as_bytes().as_ptr();
        let ptr_u8: *const u8 = mem::transmute(ptr);
        let len = source.len() as GLint;
        gl::ShaderSource(shader, 1, mem::transmute(&ptr_u8), &len);
        gl::CompileShader(shader);
    }

    let mut status = 0;
    unsafe { gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status) };
    if status != (gl::TRUE as GLint) {
        let mut len = 0;
        unsafe { gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len); }
        let mut buf = Vec::with_capacity(len as usize);
        unsafe {
            buf.set_len(len as usize);
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
        }
        panic!("{}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
    }

    shader
}
