use core::ops::Drop;

use gl;
use gl::types::*;

use collection_traits::*;
use vector::Vector;

use context::Context;
use texture::Texture;
use enums::Attachment;


pub struct Framebuffer {
    id: GLuint,
}

impl Drop for Framebuffer {
    #[inline]
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe { gl::DeleteFramebuffers(1, &self.id); }
        }
    }
}

impl Framebuffer {
    #[inline(always)]
    pub fn new() -> Self {
        Framebuffer {
            id: {
                let mut id = 0;
                unsafe { gl::GenFramebuffers(1, &mut id); }
                id
            },
        }
    }
    #[inline(always)]
    pub fn id(&self) -> GLuint { self.id }

    #[inline]
    pub fn set(&mut self, _: &Context, texture: &Texture, buffers: &[Attachment], level: GLint) {
        let texture_id = texture.id();

        let mut gl_enums = Vector::with_capacity(buffers.len());
        for i in 0..buffers.len() {
            gl_enums.push(buffers[i].to_gl());
        }

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            gl::BindTexture(texture.kind(), texture_id);

            for i in 0..gl_enums.len() {
                gl::FramebufferTexture(gl::FRAMEBUFFER, gl_enums[i], texture_id, level);
            }
            gl::DrawBuffers(buffers.len() as i32, gl_enums.as_ptr());

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Check framebuffer status failed");
            }
        }
    }
}
