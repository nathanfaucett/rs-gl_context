use core::fmt::Debug;

use gl;

use context::Context;
use enums::{TextureFormat, TextureWrap, TextureKind, FilterMode};


pub trait TextureTrait: Debug {

    fn needs_update(&self) -> bool;
    fn set_needs_update(&mut self, needs_update: bool);

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn format(&self) -> TextureFormat;
    fn kind(&self) -> TextureKind;
    fn wrap(&self) -> TextureWrap;
    fn filter(&self) -> FilterMode;

    fn generate_mipmap(&self) -> bool;
    fn flip_y(&self) -> bool;
    fn premultiply_alpha(&self) -> bool;

    fn anisotropy(&self) -> usize;
}


#[derive(Debug)]
pub struct Texture {
    needs_update: bool,
    texture: u32,
}

impl Texture {

    pub fn new() -> Self {
        Texture {
            needs_update: true,
            texture: 0u32,
        }
    }

    pub fn texture(&mut self, context: &mut Context, texture: &mut TextureTrait) -> u32 {
        self.compile(context, texture);
        self.texture
    }

    pub fn needs_update(&self, texture: &mut TextureTrait) -> bool {
        self.needs_update || self.texture == 0 || texture.needs_update()
    }

    fn compile(&mut self, context: &mut Context, texture: &mut TextureTrait) {
        if self.needs_update(texture) {
            self.needs_update = false;
            texture.set_needs_update(false);
        }
    }

    fn is_pot(&mut self, texture: &TextureTrait) -> bool {
        is_pot(texture.width()) && is_pot(texture.height())
    }

    fn format(&mut self, texture: &TextureTrait) -> u32 {
        match texture.format() {
            TextureFormat::RGBA => gl::RGBA,
            TextureFormat::RGB => gl::RGB,
            TextureFormat::Alpha => gl::ALPHA,
            TextureFormat::Luminance => gl::RGBA, //gl::LUMINANCE,
            TextureFormat::LuminanceAlpha => gl::RGBA, //gl::LUMINANCE_ALPHA,
        }
    }

    fn kind(&mut self, texture: &TextureTrait) -> u32 {
        match texture.kind() {
            TextureKind::UnsignedByte => gl::UNSIGNED_BYTE,
            TextureKind::Float => gl::FLOAT,
            TextureKind::DepthComponent => gl::DEPTH_COMPONENT,
            TextureKind::UnsignedShort => gl::UNSIGNED_SHORT,
            TextureKind::UnsignedShort565 => gl::UNSIGNED_SHORT_5_6_5,
            TextureKind::UnsignedShort4444 => gl::UNSIGNED_SHORT_4_4_4_4,
            TextureKind::UnsignedShort5551 => gl::UNSIGNED_SHORT_5_5_5_1,
        }
    }

    fn wrap(&mut self, texture: &TextureTrait) -> u32 {
        match texture.wrap() {
            TextureWrap::Repeat => gl::REPEAT,
            TextureWrap::Clamp => gl::CLAMP_TO_EDGE,
            TextureWrap::MirroredRepeat => gl::MIRRORED_REPEAT,
        }
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
