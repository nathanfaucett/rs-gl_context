use core::mem;
use core::ops::Drop;

use gl;
use gl::types::*;

use context::Context;
use enums::{gl_kind, TextureKind};


#[derive(Debug)]
pub struct Renderbuffer {
    id: GLuint,
}

impl Drop for Renderbuffer {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe { gl::DeleteRenderbuffers(1, mem::transmute(&self.id)); }
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

    pub fn set(&self, _: &Context, kind: TextureKind, width: usize, height: usize) {
        let kind = gl_kind(kind);

        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.id);
            gl::RenderbufferStorage(gl::RENDERBUFFER, kind, width as GLint, height as GLint);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, self.id);
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
        }
    }
}
