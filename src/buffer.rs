use core::mem;
use core::ops::Drop;

use gl;
use gl::types::*;

use super::enums::{BufferTarget, Usage};


pub struct Buffer {
    id: GLuint,

    stride: usize,
    kind: BufferTarget,
    usage: Usage,

    size: usize,
    kind_size: usize,
    length: usize,
}

impl Drop for Buffer {
    #[inline]
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe { gl::DeleteBuffers(1, &self.id); }
        }
    }
}

impl Buffer {

    #[inline]
    pub fn new() -> Self {
        Buffer {
            id: {
                let mut id = 0;
                unsafe { gl::GenBuffers(1, &mut id); }
                id
            },

            stride: 0,
            kind: BufferTarget::Array,
            usage: Usage::StaticDraw,

            size: 0,
            kind_size: 0,
            length: 0,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> GLuint { self.id }

    #[inline(always)]
    pub fn stride(&self) -> usize { self.stride }
    #[inline(always)]
    pub fn kind(&self) -> BufferTarget { self.kind }
    #[inline(always)]
    pub fn usage(&self) -> Usage { self.usage }

    #[inline(always)]
    pub fn size(&self) -> usize { self.size }
    #[inline(always)]
    pub fn kind_size(&self) -> usize { self.kind_size }
    #[inline(always)]
    pub fn length(&self) -> usize { self.length }

    #[inline]
    pub fn set<T>(&mut self, kind: BufferTarget, array: &[T], stride: usize, usage: Usage) -> &mut Self {
        let length = array.len();
        let kind_size = mem::size_of::<T>();
        let size = kind_size * length;

        unsafe {
            let kind = kind.to_gl();
            let usage = usage.to_gl();
            gl::BindBuffer(kind, self.id);
    		gl::BufferData(kind, size as GLsizeiptr, array.as_ptr() as *const _, usage);
    		gl::BindBuffer(kind, 0);
        };

        self.stride = stride;
        self.kind = kind;
        self.usage = usage;

        self.size = size;
        self.kind_size = kind_size;
        self.length = length;

        self
    }
}
