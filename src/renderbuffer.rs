use core::ops::Drop;

use gl;
use gl::types::*;

use context::Context;
use enums::{TextureFormat, Attachment};


#[derive(Debug)]
pub struct Renderbuffer {
    id: GLuint,
}

impl Drop for Renderbuffer {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe { gl::DeleteRenderbuffers(1, &self.id); }
        }
    }
}

impl Renderbuffer {
    pub fn new() -> Self {
        Renderbuffer {
            id: {
                let mut id = 0;
                unsafe { gl::GenRenderbuffers(1, &mut id); }
                id
            },
        }
    }
    pub fn id(&self) -> GLuint { self.id }

    pub fn set(&self, _: &Context, format: TextureFormat, attachment: Attachment, width: usize, height: usize) {
        let format = format.to_gl();
        let attachment = attachment.to_gl();

        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.id);
            gl::RenderbufferStorage(gl::RENDERBUFFER, format, width as GLint, height as GLint);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, attachment, gl::RENDERBUFFER, self.id);
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
        }
    }
}
