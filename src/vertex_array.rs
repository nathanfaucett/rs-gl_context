use core::ops::Drop;

use gl;
use gl::types::*;


#[derive(Debug)]
pub struct VertexArray {
    id: usize,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe { gl::DeleteVertexArrays(1, &(self.id as GLuint) as *const _); }
        }
    }
}

impl VertexArray {
    pub fn new() -> Self {
        VertexArray {
            id: {
                let mut id = 0;
                unsafe { gl::GenVertexArrays(1, &mut id); }
                id as usize
            },
        }
    }
    pub fn id(&self) -> usize { self.id }
}
