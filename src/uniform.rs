use std::fmt::Debug;
use std::any::Any;

use gl;
use gl::types::*;
use context::Context;
use texture::Texture;


fn copy_array<'a, 'b, T: Copy>(a: &'a mut [T], b: &'b [T]) {
    let mut i = 0usize;
    let il = b.len();

    while i < il {
        a[i] = b[i];
        i += 1;
    }
}

fn ne_array<'a, 'b, T: PartialEq>(a: &'a [T], b: &'b [T]) -> bool {
    let mut i = 0usize;
    let il = b.len();

    while i < il {
        if a[i] != b[i] {
            return true;
        } else {
            i += 1;
        }
    }
    false
}


pub trait Uniform: Debug {
    fn name(&self) -> String;
    fn kind(&self) -> GLenum;
    fn size(&self) -> usize;
    fn location(&self) -> GLint;
    fn set_unchecked(&mut self, &mut Context, &Any, bool) -> bool;
    fn set(&mut self, &mut Context, &Any, bool) -> bool;
}


macro_rules! create_simple_uniform_struct {
    ($t: ident, $kind: ident, $item_count: expr) => (
        #[derive(Debug, PartialEq)]
        pub struct $t {
            name: String,
            kind: GLenum,
            size: usize,
            location: GLint,
            value: [$kind; $item_count],
        }
        impl $t {
            pub fn new(name: String, kind: GLenum, size: usize, location: GLint) -> Self {
                $t {
                    name: name,
                    kind: kind,
                    size: size,
                    location: location,
                    value: [0 as $kind; $item_count],
                }
            }
        }
    );
}
macro_rules! create_simple_single_uniform_struct {
    ($t: ident, $kind: ident) => (
        #[derive(Debug, PartialEq)]
        pub struct $t {
            name: String,
            kind: GLenum,
            size: usize,
            location: GLint,
            value: $kind,
        }
        impl $t {
            pub fn new(name: String, kind: GLenum, size: usize, location: GLint) -> Self {
                $t {
                    name: name,
                    kind: kind,
                    size: size,
                    location: location,
                    value: 0 as $kind,
                }
            }
        }
    );
}
macro_rules! create_uniform_struct {
    ($t: ident) => (
        #[derive(Debug, PartialEq)]
        pub struct $t {
            name: String,
            kind: GLenum,
            size: usize,
            location: GLint,
        }
        impl $t {
            pub fn new(name: String, kind: GLenum, size: usize, location: GLint) -> Self {
                $t {
                    name: name,
                    kind: kind,
                    size: size,
                    location: location,
                }
            }
        }
    );
}

macro_rules! create_simple_uniform {
    ($t: ident, $func: ident, $kind: ident, $item_count: expr) => (
        impl Uniform for $t {
            fn name(&self) -> String { self.name.clone() }
            fn kind(&self) -> GLenum { self.kind }
            fn size(&self) -> usize { self.size }
            fn location(&self) -> GLint { self.location }
            fn set_unchecked(&mut self, _: &mut Context, value: &Any, _: bool) -> bool {
                match value.downcast_ref::<[$kind; $item_count]>() {
                    Some(value) => {
                        copy_array(&mut self.value, value);
                        unsafe { gl::$func(self.location, 1, value.as_ptr()); }
                        true
                    },
                    None => panic!(
                        "Invalid value passed to uniform {:?} expected {:?}",
                        self.name,
                        stringify!([$kind; $item_count])
                    ),
                }
            }
            fn set(&mut self, _: &mut Context, value: &Any, force: bool) -> bool {
                match value.downcast_ref::<[$kind; $item_count]>() {
                    Some(value) => {
                        if force || ne_array(&self.value, value) {
                            copy_array(&mut self.value, value);
                            unsafe { gl::$func(self.location, 1, value.as_ptr()); }
                            true
                        } else {
                            false
                        }
                    },
                    None => panic!(
                        "Invalid value passed to uniform {:?} expected {:?}",
                        self.name,
                        stringify!([$kind; $item_count])
                    ),
                }
            }
        }
    );
}
macro_rules! create_simple_single_uniform {
    ($t: ident, $func: ident, $kind: ident) => (
        impl Uniform for $t {
            fn name(&self) -> String { self.name.clone() }
            fn kind(&self) -> GLenum { self.kind }
            fn size(&self) -> usize { self.size }
            fn location(&self) -> GLint { self.location }
            fn set_unchecked(&mut self, _: &mut Context, value: &Any, _: bool) -> bool {
                match value.downcast_ref::<$kind>() {
                    Some(value) => {
                        self.value = *value;
                        unsafe { gl::$func(self.location, *value) };
                        true
                    },
                    None => panic!(
                        "Invalid value passed to uniform {:?} expected {:?}",
                        self.name,
                        stringify!($kind)
                    ),
                }
            }
            fn set(&mut self, _: &mut Context, value: &Any, force: bool) -> bool {
                match value.downcast_ref::<$kind>() {
                    Some(value) => {
                        if force || self.value != *value {
                            self.value = *value;
                            unsafe { gl::$func(self.location, value.clone()); }
                            true
                        } else {
                            false
                        }
                    },
                    None => panic!(
                        "Invalid value passed to uniform {:?} expected {:?}",
                        self.name,
                        stringify!($kind)
                    ),
                }
            }
        }
    );
}
macro_rules! create_matrix_uniform {
    ($t: ident, $func: ident, $kind: ident, $item_count: expr) => (
        impl Uniform for $t {
            fn name(&self) -> String { self.name.clone() }
            fn kind(&self) -> GLenum { self.kind }
            fn size(&self) -> usize { self.size }
            fn location(&self) -> GLint { self.location }
            fn set_unchecked(&mut self, _: &mut Context, value: &Any, _: bool) -> bool {
                match value.downcast_ref::<[$kind; $item_count]>() {
                    Some(value) => {
                        copy_array(&mut self.value, value);
                        unsafe { gl::$func(self.location, 1, gl::FALSE, value.as_ptr()); }
                        true
                    },
                    None => panic!(
                        "Invalid value passed to uniform {:?} expected {:?}",
                        self.name,
                        stringify!([$kind; $item_count])
                    ),
                }
            }
            fn set(&mut self, _: &mut Context, value: &Any, force: bool) -> bool {
                match value.downcast_ref::<[$kind; $item_count]>() {
                    Some(value) => {
                        if force || ne_array(&self.value, value) {
                            copy_array(&mut self.value, value);
                            unsafe { gl::$func(self.location, 1, gl::FALSE, value.as_ptr()); }
                            true
                        } else {
                            false
                        }
                    },
                    None => panic!(
                        "Invalid value passed to uniform {:?} expected {:?}",
                        self.name,
                        stringify!([$kind; $item_count])
                    ),
                }
            }
        }
    );
}


create_simple_single_uniform_struct!(Uniform1f, f32);
create_simple_single_uniform!(Uniform1f, Uniform1f, f32);
create_simple_single_uniform_struct!(Uniform1i, i32);
create_simple_single_uniform!(Uniform1i, Uniform1i, i32);

create_simple_uniform_struct!(Uniform2f, f32, 2);
create_simple_uniform!(Uniform2f, Uniform2fv, f32, 2);
create_simple_uniform_struct!(Uniform2i, i32, 2);
create_simple_uniform!(Uniform2i, Uniform2iv, i32, 2);

create_simple_uniform_struct!(Uniform3f, f32, 3);
create_simple_uniform!(Uniform3f, Uniform3fv, f32, 3);
create_simple_uniform_struct!(Uniform3i, i32, 3);
create_simple_uniform!(Uniform3i, Uniform3iv, i32, 3);

create_simple_uniform_struct!(Uniform4f, f32, 4);
create_simple_uniform!(Uniform4f, Uniform4fv, f32, 4);
create_simple_uniform_struct!(Uniform4i, i32, 4);
create_simple_uniform!(Uniform4i, Uniform4iv, i32, 4);

create_simple_uniform_struct!(UniformMatrix2f, f32, 4);
create_matrix_uniform!(UniformMatrix2f, UniformMatrix2fv, f32, 4);

create_simple_uniform_struct!(UniformMatrix3f, f32, 9);
create_matrix_uniform!(UniformMatrix3f, UniformMatrix3fv, f32, 9);

create_simple_uniform_struct!(UniformMatrix4f, f32, 16);
create_matrix_uniform!(UniformMatrix4f, UniformMatrix4fv, f32, 16);


macro_rules! create_size_simple_uniform {
    ($t: ident, $func: ident, $kind: ident, $item_count: expr) => (
        impl Uniform for $t {
            fn name(&self) -> String { self.name.clone() }
            fn kind(&self) -> GLenum { self.kind }
            fn size(&self) -> usize { self.size }
            fn location(&self) -> GLint { self.location }
            fn set_unchecked(&mut self, _: &mut Context, value: &Any, _: bool) -> bool {
                match value.downcast_ref::<[$kind; $item_count]>() {
                    Some(value) => unsafe {
                        gl::$func(
                            self.location,
                            self.size as GLint,
                            value.as_ptr()
                        );
                        true
                    },
                    None => panic!(
                        "Invalid value passed to uniform {:?} expected {:?}",
                        self.name,
                        stringify!([$kind; $item_count])
                    ),
                }
            }
            fn set(&mut self, context: &mut Context, value: &Any, force: bool) -> bool {
                self.set_unchecked(context, value, force)
            }
        }
    );
}
macro_rules! create_size_matrix_uniform {
    ($t: ident, $func: ident, $kind: ident, $item_count: expr) => (
        impl Uniform for $t {
            fn name(&self) -> String { self.name.clone() }
            fn kind(&self) -> GLenum { self.kind }
            fn size(&self) -> usize { self.size }
            fn location(&self) -> GLint { self.location }
            fn set_unchecked(&mut self, _: &mut Context, value: &Any, _: bool) -> bool {
                unsafe {
                    gl::$func(
                        self.location,
                        self.size as GLint,
                        gl::FALSE,
                        (value as *const Any) as *const $kind
                    );
                    true
                }
            }
            fn set(&mut self, context: &mut Context, value: &Any, force: bool) -> bool {
                self.set_unchecked(context, value, force)
            }
        }
    );
}

create_uniform_struct!(Uniform1fv);
create_size_simple_uniform!(Uniform1fv, Uniform1fv, f32, 1);

create_uniform_struct!(Uniform1iv);
create_size_simple_uniform!(Uniform1iv, Uniform1iv, i32, 1);

create_uniform_struct!(Uniform2fv);
create_size_simple_uniform!(Uniform2fv, Uniform2fv, f32, 2);
create_uniform_struct!(Uniform2iv);
create_size_simple_uniform!(Uniform2iv, Uniform2iv, i32, 2);

create_uniform_struct!(Uniform3fv);
create_size_simple_uniform!(Uniform3fv, Uniform3fv, f32, 3);
create_uniform_struct!(Uniform3iv);
create_size_simple_uniform!(Uniform3iv, Uniform3iv, i32, 3);

create_uniform_struct!(Uniform4fv);
create_size_simple_uniform!(Uniform4fv, Uniform4fv, f32, 4);
create_uniform_struct!(Uniform4iv);
create_size_simple_uniform!(Uniform4iv, Uniform4iv, i32, 4);

create_uniform_struct!(UniformMatrix2fv);
create_size_matrix_uniform!(UniformMatrix2fv, UniformMatrix2fv, f32, 4);

create_uniform_struct!(UniformMatrix3fv);
create_size_matrix_uniform!(UniformMatrix3fv, UniformMatrix3fv, f32, 9);

create_uniform_struct!(UniformMatrix4fv);
create_size_matrix_uniform!(UniformMatrix4fv, UniformMatrix4fv, f32, 16);


macro_rules! create_texture_uniform {
    ($t: ident) => (
        impl Uniform for $t {
            fn name(&self) -> String { self.name.clone() }
            fn kind(&self) -> GLenum { self.kind }
            fn size(&self) -> usize { self.size }
            fn location(&self) -> GLint { self.location }
            fn set_unchecked(&mut self, context: &mut Context, value: &Any, force: bool) -> bool {
                match value.downcast_ref::<Texture>() {
                    Some(texture) => {
                        context.set_texture(self.location, texture, force)
                    },
                    None => panic!(
                        "Invalid value passed to uniform {:?} expected Texture",
                        self.name
                    ),
                }
            }
            fn set(&mut self, context: &mut Context, value: &Any, force: bool) -> bool {
                match value.downcast_ref::<Texture>() {
                    Some(texture) => {
                        context.set_texture(self.location, texture, force)
                    },
                    None => panic!(
                        "Invalid value passed to uniform {:?} expected Texture",
                        self.name
                    ),
                }
            }
        }
    );
}


create_uniform_struct!(UniformTexture);
create_texture_uniform!(UniformTexture);


pub fn new_uniform(name: String, kind: GLenum, size: usize, location: GLint) -> Box<Uniform> {
    if size > 1 {
        match kind {
            gl::FLOAT => Box::new(Uniform1fv::new(name, kind, size, location)) as Box<Uniform>,
            gl::INT => Box::new(Uniform1iv::new(name, kind, size, location)) as Box<Uniform>,

            gl::FLOAT_VEC2 => Box::new(Uniform2fv::new(name, kind, size, location)) as Box<Uniform>,
            gl::INT_VEC2 => Box::new(Uniform2iv::new(name, kind, size, location)) as Box<Uniform>,

            gl::FLOAT_VEC3 => Box::new(Uniform3fv::new(name, kind, size, location)) as Box<Uniform>,
            gl::INT_VEC3 => Box::new(Uniform3iv::new(name, kind, size, location)) as Box<Uniform>,

            gl::FLOAT_VEC4 => Box::new(Uniform4fv::new(name, kind, size, location)) as Box<Uniform>,
            gl::INT_VEC4 => Box::new(Uniform4iv::new(name, kind, size, location)) as Box<Uniform>,

            gl::FLOAT_MAT2 => Box::new(UniformMatrix2fv::new(name, kind, size, location)) as Box<Uniform>,
            gl::FLOAT_MAT3 => Box::new(UniformMatrix3fv::new(name, kind, size, location)) as Box<Uniform>,
            gl::FLOAT_MAT4 => Box::new(UniformMatrix4fv::new(name, kind, size, location)) as Box<Uniform>,

            _ => panic!("Invalid uniform type {:?}", kind),
        }
    } else {
        match kind {
            gl::SAMPLER_2D => Box::new(UniformTexture::new(name, kind, size, location)) as Box<Uniform>,

            gl::FLOAT => Box::new(Uniform1f::new(name, kind, size, location)) as Box<Uniform>,
            gl::INT => Box::new(Uniform1i::new(name, kind, size, location)) as Box<Uniform>,

            gl::FLOAT_VEC2 => Box::new(Uniform2f::new(name, kind, size, location)) as Box<Uniform>,
            gl::INT_VEC2 => Box::new(Uniform2i::new(name, kind, size, location)) as Box<Uniform>,

            gl::FLOAT_VEC3 => Box::new(Uniform3f::new(name, kind, size, location)) as Box<Uniform>,
            gl::INT_VEC3 => Box::new(Uniform3i::new(name, kind, size, location)) as Box<Uniform>,

            gl::FLOAT_VEC4 => Box::new(Uniform4f::new(name, kind, size, location)) as Box<Uniform>,
            gl::INT_VEC4 => Box::new(Uniform4i::new(name, kind, size, location)) as Box<Uniform>,

            gl::FLOAT_MAT2 => Box::new(UniformMatrix2f::new(name, kind, size, location)) as Box<Uniform>,
            gl::FLOAT_MAT3 => Box::new(UniformMatrix3f::new(name, kind, size, location)) as Box<Uniform>,
            gl::FLOAT_MAT4 => Box::new(UniformMatrix4f::new(name, kind, size, location)) as Box<Uniform>,

            _ => panic!("Invalid uniform type {:?}", kind),
        }
    }
}
