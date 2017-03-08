use alloc::boxed::Box;
use collections::string::String;

use core::fmt::Debug;

use gl;
use gl::types::*;

use context::Context;
use buffer::Buffer;


pub trait Attribute: Debug {
    fn name(&self) -> String;
    fn kind(&self) -> GLenum;
    fn size(&self) -> usize;
    fn location(&self) -> GLint;
    fn set(&self, &mut Context, &Buffer, usize, bool) -> bool;
}


macro_rules! create_attribute_struct {
    ($t: ident, $size: expr, $kind: expr) => (
        #[derive(Debug)]
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
        impl Attribute for $t {
            fn name(&self) -> String { self.name.clone() }
            fn kind(&self) -> GLenum { self.kind }
            fn size(&self) -> usize { self.size }
            fn location(&self) -> GLint { self.location }
            fn set(&self, context: &mut Context, buffer: &Buffer, offset: usize, force: bool) -> bool {
                let kind_size = buffer.kind_size();

                context.set_buffer(buffer, force);
                context.set_attrib_pointer(
                    self.location() as GLuint,
                    $size,
                    $kind,
                    (buffer.stride() * kind_size) as GLint,
                    (offset * kind_size) as GLint,
                    force
                )
            }
        }
    );
}

create_attribute_struct!(Attribute1f, 1, gl::FLOAT);
create_attribute_struct!(Attribute1b, 1, gl::BOOL);
create_attribute_struct!(Attribute1i, 1, gl::INT);

create_attribute_struct!(Attribute2f, 2, gl::FLOAT);
create_attribute_struct!(Attribute2b, 2, gl::BOOL);
create_attribute_struct!(Attribute2i, 2, gl::INT);

create_attribute_struct!(Attribute3f, 3, gl::FLOAT);
create_attribute_struct!(Attribute3b, 3, gl::BOOL);
create_attribute_struct!(Attribute3i, 3, gl::INT);

create_attribute_struct!(Attribute4f, 4, gl::FLOAT);
create_attribute_struct!(Attribute4b, 4, gl::BOOL);
create_attribute_struct!(Attribute4i, 4, gl::INT);


pub fn new_attribute(name: String, kind: GLenum, size: usize, location: GLint) -> Box<Attribute> {
    match kind {
        gl::BOOL => Box::new(Attribute1i::new(name, kind, size, location)) as Box<Attribute>,
        gl::INT => Box::new(Attribute1i::new(name, kind, size, location)) as Box<Attribute>,
        gl::FLOAT => Box::new(Attribute1f::new(name, kind, size, location)) as Box<Attribute>,

        gl::BOOL_VEC2 => Box::new(Attribute2i::new(name, kind, size, location)) as Box<Attribute>,
        gl::INT_VEC2 => Box::new(Attribute2i::new(name, kind, size, location)) as Box<Attribute>,
        gl::FLOAT_VEC2 => Box::new(Attribute2f::new(name, kind, size, location)) as Box<Attribute>,

        gl::BOOL_VEC3 => Box::new(Attribute3i::new(name, kind, size, location)) as Box<Attribute>,
        gl::INT_VEC3 => Box::new(Attribute3i::new(name, kind, size, location)) as Box<Attribute>,
        gl::FLOAT_VEC3 => Box::new(Attribute3f::new(name, kind, size, location)) as Box<Attribute>,

        gl::BOOL_VEC4 => Box::new(Attribute4i::new(name, kind, size, location)) as Box<Attribute>,
        gl::INT_VEC4 => Box::new(Attribute4i::new(name, kind, size, location)) as Box<Attribute>,
        gl::FLOAT_VEC4 => Box::new(Attribute4f::new(name, kind, size, location)) as Box<Attribute>,

        _ => panic!("Invalid attribte type {:?}", kind),
    }
}
