use core::mem;
use core::ops::Drop;

use gl;
use gl::types::*;

use context::Context;
use enums::{TextureFormat, TextureWrap, TextureKind, FilterMode};


#[derive(Debug)]
pub struct Texture {
    id: GLuint,
}

impl Drop for Texture {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe { gl::DeleteTextures(1, mem::transmute(&self.id)); }
        }
    }
}

impl Texture {

    pub fn new() -> Self {
        Texture {
            id: {
                let mut id = 0;
                unsafe { gl::GenTextures(1, &mut id); }
                id
            },
        }
    }

    pub fn get_id(&self) -> GLuint { self.id }

    pub fn set<T>(
        &mut self,
        context: &Context,
        width: usize,
        height: usize,
        format: TextureFormat,
        kind: TextureKind,
        wrap: TextureWrap,
        filter: FilterMode,
        generate_mipmap: bool,
        data: &[T],
    ) -> &mut Self {
        let pot = is_pot(width as usize) && is_pot(height as usize);

        let major = context.get_major();
        let minor = context.get_minor();

        let mag_filter;
        let min_filter;

        if filter == FilterMode::None {
            mag_filter = gl::NEAREST;
            min_filter = if pot && generate_mipmap {gl::LINEAR_MIPMAP_NEAREST} else {gl::NEAREST};
        } else {
            mag_filter = gl::LINEAR;
            min_filter = if pot && generate_mipmap {gl::LINEAR_MIPMAP_LINEAR} else {gl::LINEAR};
        }

        let format = get_format(format) ;
        let wrap = get_wrap(wrap) as GLint;
        let kind = get_kind(kind);

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id as GLuint);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter as GLint);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap);

            gl::TexImage2D(gl::TEXTURE_2D, 0, format as GLint, width as i32, height as i32, 0, format, kind, data.as_ptr() as *const _);

            if generate_mipmap && pot {
                if major >= 4 && minor >= 5 {
                    gl::GenerateTextureMipmap(self.id as GLuint);
                } else {
                    gl::GenerateMipmap(gl::TEXTURE_2D);
                }
            }

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        self
    }
}

fn get_format(format: TextureFormat) -> GLenum {
    match format {
        TextureFormat::RGB => gl::RGB,
        TextureFormat::RGBA => gl::RGBA,
        TextureFormat::Alpha => gl::ALPHA,
        TextureFormat::Luminance => gl::RGBA, //gl::LUMINANCE,
        TextureFormat::LuminanceAlpha => gl::RGBA, //gl::LUMINANCE_ALPHA,
    }
}

fn get_kind(kind: TextureKind) -> GLenum {
    match kind {
        TextureKind::UnsignedByte => gl::UNSIGNED_BYTE,
        TextureKind::Float => gl::FLOAT,
        TextureKind::DepthComponent => gl::DEPTH_COMPONENT,
        TextureKind::UnsignedShort => gl::UNSIGNED_SHORT,
        TextureKind::UnsignedShort565 => gl::UNSIGNED_SHORT_5_6_5,
        TextureKind::UnsignedShort4444 => gl::UNSIGNED_SHORT_4_4_4_4,
        TextureKind::UnsignedShort5551 => gl::UNSIGNED_SHORT_5_5_5_1,
    }
}

fn get_wrap(wrap: TextureWrap) -> GLenum {
    match wrap {
        TextureWrap::Repeat => gl::REPEAT,
        TextureWrap::Clamp => gl::CLAMP_TO_EDGE,
        TextureWrap::MirroredRepeat => gl::MIRRORED_REPEAT,
    }
}

fn is_pot(x: usize) -> bool {
    !(
        x != 1 && x != 2 && x != 4 && x != 8 && x != 16 && x != 32 &&
        x != 64 && x != 128 && x != 256 && x != 512 && x != 1024 &&
        x != 2048 && x != 4096 && x != 8192 && x != 16384 &&
        x != 32768 && x != 65536 && x != 131072 && x != 262144 &&
        x != 524288 && x != 1048576 && x != 2097152 &&
        x != 4194304 && x != 8388608 && x != 16777216 &&
        x != 33554432 && x != 67108864 && x != 134217728 &&
        x != 268435456 && x != 536870912 && x != 1073741824 &&
        x != 2147483648
    )
}
