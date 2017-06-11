use gl;
use gl::types::*;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Blending {
    None,
    Default,
    Additive,
    Subtractive,
    Multiply,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CullFace {
    None,
    Back,
    Front,
    FrontAndBack
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FilterMode {
    None,
    Linear,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DrawMode {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    LineStripAdjacency,
    LineAdjacency,
    TriangleStrip,
    TriangleFan,
    Triangles,
    TriangleStripAdjacency,
    TrianglesAdjacency,
    Patches,
}

impl DrawMode {
    #[inline]
    pub fn to_gl(&self) -> GLenum {
        match self {
            &DrawMode::Points => gl::POINTS,
            &DrawMode::LineStrip => gl::LINE_STRIP,
            &DrawMode::LineLoop => gl::LINE_LOOP,
            &DrawMode::Lines => gl::LINES,
            &DrawMode::LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
            &DrawMode::LineAdjacency => gl::LINES_ADJACENCY,
            &DrawMode::TriangleStrip => gl::TRIANGLE_STRIP,
            &DrawMode::TriangleFan => gl::TRIANGLE_FAN,
            &DrawMode::Triangles => gl::TRIANGLES,
            &DrawMode::TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
            &DrawMode::TrianglesAdjacency => gl::TRIANGLES_ADJACENCY,
            &DrawMode::Patches => gl::PATCHES,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IndexKind {
    UnsignedByte,
    UnsignedShort,
    UnsignedInt,
}

impl IndexKind {
    #[inline]
    pub fn to_gl(&self) -> GLenum {
        match self {
            &IndexKind::UnsignedByte => gl::UNSIGNED_BYTE,
            &IndexKind::UnsignedShort => gl::UNSIGNED_SHORT,
            &IndexKind::UnsignedInt => gl::UNSIGNED_INT,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Usage {
    StreamDraw, StreamRead, StreamCopy,
    StaticDraw, StaticRead, StaticCopy,
    DynamicDraw, DynamicRead, DynamicCopy,
}

impl Usage {
    #[inline]
    pub fn to_gl(&self) -> GLenum {
        match self {
            &Usage::StreamDraw => gl::STREAM_DRAW,
            &Usage::StreamRead => gl::STREAM_READ,
            &Usage::StreamCopy => gl::STREAM_COPY,

            &Usage::StaticDraw => gl::STATIC_DRAW,
            &Usage::StaticRead => gl::STATIC_READ,
            &Usage::StaticCopy => gl::STATIC_COPY,

            &Usage::DynamicDraw => gl::DYNAMIC_DRAW,
            &Usage::DynamicRead => gl::DYNAMIC_READ,
            &Usage::DynamicCopy => gl::DYNAMIC_COPY,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BufferTarget {
    Array,
    AtomicCounter,
    CopyRead,
    CopyWrite,
    DispatchIndirect,
    DrawIndirect,
    ElementArray,
    PixelPack,
    PixelUnpack,
    Query,
    ShaderStorage,
    Texture,
    TransformFeedback,
    Uniform,
}

impl BufferTarget {
    #[inline]
    pub fn to_gl(&self) -> GLenum {
        match self {
            &BufferTarget::Array => gl::ARRAY_BUFFER,
            &BufferTarget::AtomicCounter => gl::ATOMIC_COUNTER_BUFFER,
            &BufferTarget::CopyRead => gl::COPY_READ_BUFFER,
            &BufferTarget::CopyWrite => gl::COPY_WRITE_BUFFER,
            &BufferTarget::DispatchIndirect => gl::DISPATCH_INDIRECT_BUFFER,
            &BufferTarget::DrawIndirect => gl::DRAW_INDIRECT_BUFFER,
            &BufferTarget::ElementArray => gl::ELEMENT_ARRAY_BUFFER,
            &BufferTarget::PixelPack => gl::PIXEL_PACK_BUFFER,
            &BufferTarget::PixelUnpack => gl::PIXEL_UNPACK_BUFFER,
            &BufferTarget::Query => gl::QUERY_BUFFER,
            &BufferTarget::ShaderStorage => gl::SHADER_STORAGE_BUFFER,
            &BufferTarget::Texture => gl::TEXTURE_BUFFER,
            &BufferTarget::TransformFeedback => gl::TRANSFORM_FEEDBACK_BUFFER,
            &BufferTarget::Uniform => gl::UNIFORM_BUFFER,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Attachment {
    Color,
    Depth,
    Stencil
}

impl Attachment {
    #[inline]
    pub fn to_gl(&self) -> GLenum {
        match self {
            &Attachment::Color => gl::COLOR_ATTACHMENT0,
            &Attachment::Depth => gl::DEPTH_ATTACHMENT,
            &Attachment::Stencil => gl::STENCIL_ATTACHMENT,
        }
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TextureFormat {
    Red, RG, RGB, BGR, RGBA, BGRA,
    RedInteger, RGInteger, RGBInteger, BGRInteger, RGBAInteger, BGRAInteger,
    StencilInteger,
    DepthComponent,
    DepthStencil,
}

impl TextureFormat {
    #[inline]
    pub fn to_gl(&self) -> GLenum {
        match self {
            &TextureFormat::Red => gl::RED,
            &TextureFormat::RG => gl::RG,
            &TextureFormat::RGB => gl::RGB,
            &TextureFormat::BGR => gl::BGR,
            &TextureFormat::RGBA => gl::RGBA,
            &TextureFormat::BGRA => gl::BGRA,
            &TextureFormat::RedInteger => gl::RED_INTEGER,
            &TextureFormat::RGInteger => gl::RG_INTEGER,
            &TextureFormat::RGBInteger => gl::RGB_INTEGER,
            &TextureFormat::BGRInteger => gl::BGR_INTEGER,
            &TextureFormat::RGBAInteger => gl::RGBA_INTEGER,
            &TextureFormat::BGRAInteger => gl::BGRA_INTEGER,
            &TextureFormat::StencilInteger => gl::STENCIL_INDEX,
            &TextureFormat::DepthComponent => gl::DEPTH_COMPONENT,
            &TextureFormat::DepthStencil => gl::DEPTH_STENCIL,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TextureKind {
    UnsignedByte,
    Byte,
    UnsignedShort,
    Short,
    UnsignedInt,
    Int,
    Float,

    UnsignedByte332,
    UnsignedByte223Rev,

    UnsignedShort565,
    UnsignedShort565Rev,

    UnsignedShort4444,
    UnsignedShort4444Rev,

    UnsignedShort5551,
    UnsignedShort1555Rev,

    UnsignedInt8888,
    UnsignedInt8888Rev,

    UnsignedInt1010102,
    UnsignedInt2101010Rev,
}

impl TextureKind {
    #[inline]
    pub fn to_gl(&self) -> GLenum {
        match self {
            &TextureKind::UnsignedByte => gl::UNSIGNED_BYTE,
            &TextureKind::Byte => gl::BYTE,
            &TextureKind::UnsignedShort => gl::UNSIGNED_SHORT,
            &TextureKind::Short => gl::SHORT,
            &TextureKind::UnsignedInt => gl::UNSIGNED_INT,
            &TextureKind::Int => gl::INT,
            &TextureKind::Float => gl::FLOAT,

            &TextureKind::UnsignedByte332 => gl::UNSIGNED_BYTE_3_3_2,
            &TextureKind::UnsignedByte223Rev => gl::UNSIGNED_BYTE_2_3_3_REV,

            &TextureKind::UnsignedShort565 => gl::UNSIGNED_SHORT_5_6_5,
            &TextureKind::UnsignedShort565Rev => gl::UNSIGNED_SHORT_5_6_5_REV,

            &TextureKind::UnsignedShort4444 => gl::UNSIGNED_SHORT_4_4_4_4,
            &TextureKind::UnsignedShort4444Rev => gl::UNSIGNED_SHORT_4_4_4_4_REV,

            &TextureKind::UnsignedShort5551 => gl::UNSIGNED_SHORT_5_5_5_1,
            &TextureKind::UnsignedShort1555Rev => gl::UNSIGNED_SHORT_1_5_5_5_REV,

            &TextureKind::UnsignedInt8888 => gl::UNSIGNED_INT_8_8_8_8,
            &TextureKind::UnsignedInt8888Rev => gl::UNSIGNED_INT_8_8_8_8_REV,

            &TextureKind::UnsignedInt1010102 => gl::UNSIGNED_INT_10_10_10_2,
            &TextureKind::UnsignedInt2101010Rev => gl::UNSIGNED_INT_2_10_10_10_REV,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TextureWrap {
    Repeat,
    Clamp,
    MirroredRepeat,
}

impl TextureWrap {
    #[inline]
    pub fn to_gl(&self) -> GLenum {
        match self {
            &TextureWrap::Repeat => gl::REPEAT,
            &TextureWrap::Clamp => gl::CLAMP_TO_EDGE,
            &TextureWrap::MirroredRepeat => gl::MIRRORED_REPEAT,
        }
    }

}
