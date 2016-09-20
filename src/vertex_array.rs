use core::mem;
use core::ops::Drop;

use gl;
use gl::types::*;


#[derive(Debug)]
pub struct VertexArray {
    id: GLuint,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe { gl::DeleteVertexArrays(1, mem::transmute(&self.id)); }
        }
    }
}

impl VertexArray {
    pub fn new() -> Self {
        VertexArray {
            id: {
                let mut id = 0;
                unsafe { gl::GenVertexArrays(1, &mut id); }
                id
            },
        }
    }
    pub fn get_id(&self) -> GLuint { self.id }
}
