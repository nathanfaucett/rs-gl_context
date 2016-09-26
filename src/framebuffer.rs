use core::mem;
use core::ops::Drop;

use gl;
use gl::types::*;

use context::Context;
use texture::Texture;


#[derive(Debug)]
pub struct Framebuffer {
    id: GLuint,
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe { gl::DeleteFramebuffers(1, mem::transmute(&self.id)); }
        }
    }
}

impl Framebuffer {
    pub fn new() -> Self {
        Framebuffer {
            id: {
                let mut id = 0;
                unsafe { gl::GenFramebuffers(1, &mut id); }
                id
            },
        }
    }
    pub fn get_id(&self) -> GLuint { self.id }

    pub fn set(&mut self, _: &Context, texture: &Texture, buffers: &[GLenum], level: GLint) {
        let texture_id = texture.get_id();

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            gl::BindTexture(texture.get_kind(), texture_id);

            for i in 0..buffers.len() {
                gl::FramebufferTexture(gl::FRAMEBUFFER, buffers[i], texture_id, level);
            }
            gl::DrawBuffers(buffers.len() as i32, buffers.as_ptr());

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Check framebuffer status failed");
            }
        }
    }
}
