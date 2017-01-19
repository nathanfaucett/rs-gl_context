use gl;
use gl::types::*;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Blending {
    None,
    Default,
    Additive,
    Subtractive,
    Multiply,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CullFace {
    None,
    Back,
    Front,
    FrontAndBack
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Depth {
    None,
    Never,
    LessThan,
    Equal,
    LessThanOrEqual,
    GreaterThan,
    NotEqual,
    GreaterThanOrEqual,
    Always,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FilterMode {
    None,
    Linear,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TextureFormat {
    RGB,
    RGBA,
    Alpha,
    Luminance,
    LuminanceAlpha,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TextureKind {
    UnsignedByte,
    Float,
    DepthComponent,
    UnsignedShort,
    UnsignedShort565,
    UnsignedShort4444,
    UnsignedShort5551,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TextureWrap {
    Repeat,
    Clamp,
    MirroredRepeat,
}


pub fn gl_format(format: TextureFormat) -> GLenum {
    match format {
        TextureFormat::RGB => gl::RGB,
        TextureFormat::RGBA => gl::RGBA,
        TextureFormat::Alpha => gl::ALPHA,
        TextureFormat::Luminance => gl::RGBA, //gl::LUMINANCE,
        TextureFormat::LuminanceAlpha => gl::RGBA, //gl::LUMINANCE_ALPHA,
    }
}

pub fn gl_kind(kind: TextureKind) -> GLenum {
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

pub fn gl_wrap(wrap: TextureWrap) -> GLenum {
    match wrap {
        TextureWrap::Repeat => gl::REPEAT,
        TextureWrap::Clamp => gl::CLAMP_TO_EDGE,
        TextureWrap::MirroredRepeat => gl::MIRRORED_REPEAT,
    }
}
