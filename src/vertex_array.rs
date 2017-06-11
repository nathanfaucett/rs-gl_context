use core::ops::Drop;

use gl;
use gl::types::*;


pub struct VertexArray {
    id: GLuint,
}

impl Drop for VertexArray {
    #[inline]
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe { gl::DeleteVertexArrays(1, &self.id); }
        }
    }
}

impl VertexArray {
    #[inline(always)]
    pub fn new() -> Self {
        VertexArray {
            id: {
                let mut id = 0;
                unsafe { gl::GenVertexArrays(1, &mut id); }
                id
            },
        }
    }
    #[inline(always)]
    pub fn id(&self) -> GLuint { self.id }
}
