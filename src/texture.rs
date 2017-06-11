use core::ptr;
use core::mem;
use core::ops::Drop;

use gl;
use gl::types::*;

use context::Context;
use enums::{TextureFormat, TextureWrap, TextureKind, FilterMode};


pub struct Texture {
    id: GLuint,
    kind: GLenum,
}

impl Drop for Texture {
    #[inline]
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe { gl::DeleteTextures(1, &self.id); }
        }
    }
}

macro_rules! texture_options {
    (
        $context: ident,
        $width: ident,
        $height: ident,
        $format: ident,
        $kind: ident,
        $wrap: ident,
        $filter: ident,
        $generate_mipmap: ident,

        $gl_major: ident,
        $gl_minor: ident,

        $gl_pot: ident,
        $gl_format: ident,
        $gl_kind: ident,
        $gl_wrap: ident,
        $gl_mag_filter: ident,
        $gl_min_filter: ident
    ) => (
        let $gl_pot = $width.is_power_of_two() && $height.is_power_of_two();

        let $gl_major = $context.major();
        let $gl_minor = $context.minor();

        let $gl_mag_filter;
        let $gl_min_filter;

        if $filter == FilterMode::None {
            $gl_mag_filter = gl::NEAREST;
            $gl_min_filter = if $gl_pot && $generate_mipmap {gl::LINEAR_MIPMAP_NEAREST} else {gl::NEAREST};
        } else {
            $gl_mag_filter = gl::LINEAR;
            $gl_min_filter = if $gl_pot && $generate_mipmap {gl::LINEAR_MIPMAP_LINEAR} else {gl::LINEAR};
        }

        let $gl_format = $format.to_gl();
        let $gl_wrap = $wrap.to_gl() as GLint;
        let $gl_kind = $kind.to_gl();
    )
}

macro_rules! generate_mipmap {
    (
        $id: expr,
        $generate_mipmap: ident,
        $gl_major: ident,
        $gl_minor: ident,
        $gl_pot: ident
    ) => (
        if $generate_mipmap && $gl_pot {
            if $gl_major >= 4 && $gl_minor >= 5 {
                gl::GenerateTextureMipmap($id);
            } else {
                gl::GenerateMipmap(gl::TEXTURE_2D);
            }
        }
    )
}


impl Texture {

    #[inline(always)]
    pub fn new() -> Self {
        Texture {
            id: {
                let mut id = 0;
                unsafe { gl::GenTextures(1, &mut id); }
                id
            },
            kind: gl::TEXTURE_2D,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> GLuint { self.id }

    #[inline(always)]
    pub fn kind(&self) -> GLenum { self.kind }

    #[inline]
    pub fn set_data2d<T>(
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
        texture_options!(
            context, width, height, format, kind, wrap, filter, generate_mipmap,
            gl_major, gl_minor,
            gl_pot, gl_format, gl_kind, gl_wrap, gl_mag_filter, gl_min_filter
        );

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id as GLuint);

            gl::TexImage2D(
                gl::TEXTURE_2D, 0, gl_format as GLint, width as GLsizei, height as GLsizei,
                0, gl_format, gl_kind, mem::transmute(data.as_ptr())
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl_mag_filter as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl_min_filter as GLint);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl_wrap);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl_wrap);

            generate_mipmap!(self.id, generate_mipmap, gl_major, gl_minor, gl_pot);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        self.kind = gl::TEXTURE_2D;

        self
    }

    #[inline]
    pub fn set_null2d(
        &mut self,
        context: &Context,
        width: usize,
        height: usize,
        format: TextureFormat,
        kind: TextureKind,
        wrap: TextureWrap,
        filter: FilterMode,
        generate_mipmap: bool
    ) -> &mut Self {
        texture_options!(
            context, width, height, format, kind, wrap, filter, generate_mipmap,
            gl_major, gl_minor,
            gl_pot, gl_format, gl_kind, gl_wrap, gl_mag_filter, gl_min_filter
        );

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id as GLuint);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl_format as GLint, width as GLsizei, height as GLsizei, 0, gl_format, gl_kind, ptr::null());

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl_mag_filter as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl_min_filter as GLint);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl_wrap);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl_wrap);

            generate_mipmap!(self.id, generate_mipmap, gl_major, gl_minor, gl_pot);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        self.kind = gl::TEXTURE_2D;

        self
    }
}
