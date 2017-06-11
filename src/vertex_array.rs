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
            unsafe { gl::DeleteVertexArrays(1, &self.id); }
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
    pub fn id(&self) -> GLuint { self.id }
}
