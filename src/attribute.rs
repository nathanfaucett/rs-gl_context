use core::fmt::Debug;
use collections::boxed::Box;
use collections::string::String;

use gl;
use gl::types::*;
use context::Context;
use buffer::Buffer;


pub trait Attribute: Debug {
    fn name(&self) -> String;
    fn kind(&self) -> GLenum;
    fn location(&self) -> usize;
    fn set(&self, context: &mut Context, buffer: &Buffer, offset: usize, force: bool) -> bool;
}


macro_rules! create_attribute_struct {
    ($t: ident, $size: expr, $kind: expr) => (
        #[derive(Debug)]
        pub struct $t {
            name: String,
            kind: GLenum,
            location: usize,
        }
        impl $t {
            pub fn new(name: String, kind: GLenum, location: usize) -> Self {
                $t {
                    name: name,
                    kind: kind,
                    location: location,
                }
            }
        }
        impl Attribute for $t {
            fn name(&self) -> String { self.name.clone() }
            fn kind(&self) -> GLenum { self.kind }
            fn location(&self) -> usize { self.location }
            fn set(&self, context: &mut Context, buffer: &Buffer, offset: usize, force: bool) -> bool {
                let kind_size = buffer.kind_size();

                context.set_buffer(buffer);

                context.set_attrib_pointer(
                    self.location,
                    $size,
                    $kind,
                    buffer.stride() * kind_size,
                    offset * kind_size,
                    force
                )
            }
        }
    );
}

create_attribute_struct!(Attribute1i, 1, gl::INT);
create_attribute_struct!(Attribute1f, 1, gl::FLOAT);
create_attribute_struct!(Attribute2i, 2, gl::INT);
create_attribute_struct!(Attribute2f, 2, gl::FLOAT);
create_attribute_struct!(Attribute3i, 3, gl::INT);
create_attribute_struct!(Attribute3f, 3, gl::FLOAT);
create_attribute_struct!(Attribute4i, 4, gl::INT);
create_attribute_struct!(Attribute4f, 4, gl::FLOAT);


pub fn new_attribute(name: String, kind: GLenum, location: usize) -> Box<Attribute> {
    match kind {
        gl::INT => Box::new(Attribute1i::new(name, kind, location)) as Box<Attribute>,
        gl::FLOAT => Box::new(Attribute1f::new(name, kind, location)) as Box<Attribute>,

        gl::INT_VEC2 => Box::new(Attribute2i::new(name, kind, location)) as Box<Attribute>,
        gl::FLOAT_VEC2 => Box::new(Attribute2f::new(name, kind, location)) as Box<Attribute>,

        gl::INT_VEC3 => Box::new(Attribute3i::new(name, kind, location)) as Box<Attribute>,
        gl::FLOAT_VEC3 => Box::new(Attribute3f::new(name, kind, location)) as Box<Attribute>,

        gl::INT_VEC4 => Box::new(Attribute4i::new(name, kind, location)) as Box<Attribute>,
        gl::FLOAT_VEC4 => Box::new(Attribute4f::new(name, kind, location)) as Box<Attribute>,

        _ => Box::new(Attribute1i::new(name, kind, location)) as Box<Attribute>,
    }
}
