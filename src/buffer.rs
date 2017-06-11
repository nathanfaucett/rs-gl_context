use core::mem;
use core::ops::Drop;

use gl;
use gl::types::*;


#[derive(Debug)]
pub struct Buffer {
    id: GLuint,

    stride: usize,
    kind: GLenum,
    draw: GLenum,

    size: usize,
    kind_size: usize,
    length: usize,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe { gl::DeleteBuffers(1, &self.id); }
        }
    }
}

impl Buffer {

    pub fn new() -> Self {
        Buffer {
            id: {
                let mut id = 0;
                unsafe { gl::GenBuffers(1, &mut id); }
                id
            },

            stride: 0,
            kind: 0,
            draw: 0,

            size: 0,
            kind_size: 0,
            length: 0,
        }
    }

    pub fn id(&self) -> GLuint { self.id }

    pub fn stride(&self) -> usize { self.stride }
    pub fn kind(&self) -> GLenum { self.kind }
    pub fn draw(&self) -> GLenum { self.draw }

    pub fn size(&self) -> usize { self.size }
    pub fn kind_size(&self) -> usize { self.kind_size }
    pub fn length(&self) -> usize { self.length }

    pub fn set<T>(&mut self, kind: GLenum, array: &[T], stride: usize, draw: GLenum) -> &mut Self {
        let length = array.len();
        let kind_size = mem::size_of::<T>();
        let size = kind_size * length;

        unsafe {
            gl::BindBuffer(kind, self.id);
    		gl::BufferData(kind, size as GLsizeiptr, array.as_ptr() as *const _, draw);
    		gl::BindBuffer(kind, 0);
        };

        self.stride = stride;
        self.kind = kind;
        self.draw = draw;

        self.size = size;
        self.kind_size = kind_size;
        self.length = length;

        self
    }
}
