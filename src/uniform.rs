use core::fmt::Debug;
use core::any::Any;
use collections::boxed::Box;
use collections::string::String;

use gl;
use gl::types::*;
use context::Context;


pub trait Uniform: Debug {
    fn name(&self) -> String;
    fn kind(&self) -> GLenum;
    fn location(&self) -> usize;
    fn set_unchecked(&mut self, context: &Context, value: &Any, force: bool) -> bool;
    fn set(&mut self, context: &Context, value: &Any, force: bool) -> bool;
}


macro_rules! create_uniform_struct {
    ($t: ident, $kind: ident, $size: expr) => (
        #[derive(Debug)]
        pub struct $t {
            name: String,
            kind: GLenum,
            location: usize,
            value: Option<[$kind; $size]>,
        }
        impl $t {
            pub fn new(name: String, kind: GLenum, location: usize) -> Self {
                $t {
                    name: name,
                    kind: kind,
                    location: location,
                    value: None,
                }
            }
        }
    );
}
macro_rules! create_simple_size_uniform {
    ($t: ident, $func: ident, $kind: ident, $size: expr) => (
        impl Uniform for $t {
            fn name(&self) -> String { self.name.clone() }
            fn kind(&self) -> GLenum { self.kind }
            fn location(&self) -> usize { self.location }
            fn set_unchecked(&mut self, context: &Context, value: &Any, force: bool) -> bool {
                match value.downcast_ref::<[$kind; $size]>() {
                    Some(value) => {
                        self.value = Some(value.clone());
                        unsafe { gl::$func(self.location as i32, $size, value as *const _); }
                        true
                    },
                    None => false,
                }
            }
            fn set(&mut self, context: &Context, value: &Any, force: bool) -> bool {
                match value.downcast_ref::<[$kind; $size]>() {
                    Some(value) => {
                        if let Some(v) = self.value {
                            if force || v != *value {
                                self.value = Some(value.clone());
                                unsafe { gl::$func(self.location as i32, $size, value as *const _); }
                                true
                            } else {
                                false
                            }
                        } else {
                            unsafe { gl::$func(self.location as i32, $size, value as *const _); }
                            true
                        }
                    },
                    None => false,
                }
            }
        }
    );
}
macro_rules! create_matrix_uniform {
    ($t: ident, $func: ident, $kind: ident, $size: expr) => (
        impl Uniform for $t {
            fn name(&self) -> String { self.name.clone() }
            fn kind(&self) -> GLenum { self.kind }
            fn location(&self) -> usize { self.location }
            fn set_unchecked(&mut self, context: &Context, value: &Any, force: bool) -> bool {
                match value.downcast_ref::<[$kind; $size]>() {
                    Some(value) => {
                        self.value = Some(value.clone());
                        unsafe { gl::$func(self.location as i32, 1, gl::FALSE, value as *const _); }
                        true
                    },
                    None => false,
                }
            }
            fn set(&mut self, context: &Context, value: &Any, force: bool) -> bool {
                match value.downcast_ref::<[$kind; $size]>() {
                    Some(value) => {
                        if let Some(v) = self.value {
                            if force || v != *value {
                                self.value = Some(value.clone());
                                unsafe { gl::$func(self.location as i32, 1, gl::FALSE, value as *const _); }
                                true
                            } else {
                                false
                            }
                        } else {
                            unsafe { gl::$func(self.location as i32, 1, gl::FALSE, value as *const _); }
                            true
                        }
                    },
                    None => false,
                }
            }
        }
    );
}

create_uniform_struct!(Uniform1i, i32, 1);
create_simple_size_uniform!(Uniform1i, Uniform1iv, i32, 1);
create_uniform_struct!(Uniform1f, f32, 1);
create_simple_size_uniform!(Uniform1f, Uniform1fv, f32, 1);

create_uniform_struct!(Uniform2i, i32, 2);
create_simple_size_uniform!(Uniform2i, Uniform2iv, i32, 2);
create_uniform_struct!(Uniform2f, f32, 2);
create_simple_size_uniform!(Uniform2f, Uniform2fv, f32, 2);

create_uniform_struct!(Uniform3i, i32, 3);
create_simple_size_uniform!(Uniform3i, Uniform3iv, i32, 3);
create_uniform_struct!(Uniform3f, f32, 3);
create_simple_size_uniform!(Uniform3f, Uniform3fv, f32, 3);

create_uniform_struct!(Uniform4i, i32, 4);
create_simple_size_uniform!(Uniform4i, Uniform4iv, i32, 4);
create_uniform_struct!(Uniform4f, f32, 4);
create_simple_size_uniform!(Uniform4f, Uniform4fv, f32, 4);

create_uniform_struct!(UniformMatrix2f, f32, 4);
create_matrix_uniform!(UniformMatrix2f, UniformMatrix2fv, f32, 4);

create_uniform_struct!(UniformMatrix3f, f32, 9);
create_matrix_uniform!(UniformMatrix3f, UniformMatrix3fv, f32, 9);

create_uniform_struct!(UniformMatrix4f, f32, 16);
create_matrix_uniform!(UniformMatrix4f, UniformMatrix4fv, f32, 16);


pub fn new_uniform(name: String, kind: GLenum, location: usize) -> Box<Uniform> {
    match kind {
        gl::INT => Box::new(Uniform1i::new(name, kind, location)) as Box<Uniform>,
        gl::FLOAT => Box::new(Uniform1f::new(name, kind, location)) as Box<Uniform>,

        gl::INT_VEC2 => Box::new(Uniform2i::new(name, kind, location)) as Box<Uniform>,
        gl::FLOAT_VEC2 => Box::new(Uniform2f::new(name, kind, location)) as Box<Uniform>,

        gl::INT_VEC3 => Box::new(Uniform3i::new(name, kind, location)) as Box<Uniform>,
        gl::FLOAT_VEC3 => Box::new(Uniform3f::new(name, kind, location)) as Box<Uniform>,

        gl::INT_VEC4 => Box::new(Uniform4i::new(name, kind, location)) as Box<Uniform>,
        gl::FLOAT_VEC4 => Box::new(Uniform4f::new(name, kind, location)) as Box<Uniform>,

        gl::FLOAT_MAT2 => Box::new(UniformMatrix2f::new(name, kind, location)) as Box<Uniform>,
        gl::FLOAT_MAT3 => Box::new(UniformMatrix3f::new(name, kind, location)) as Box<Uniform>,
        gl::FLOAT_MAT4 => Box::new(UniformMatrix4f::new(name, kind, location)) as Box<Uniform>,

        _ => Box::new(Uniform1i::new(name, kind, location)) as Box<Uniform>,
    }
}
