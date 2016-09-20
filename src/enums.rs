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
