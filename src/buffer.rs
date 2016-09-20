use core::mem;
use core::ops::Drop;

use gl;
use gl::types::*;


#[derive(Debug)]
pub struct Buffer {
    _id: GLuint,

    _stride: usize,
    _kind: GLenum,
    _draw: GLenum,

    _size: usize,
    _kind_size: usize,
    _length: usize,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if self._id != 0 {
            unsafe { gl::DeleteBuffers(1, mem::transmute(&self._id)); }
        }
    }
}

impl Buffer {

    pub fn new() -> Self {
        Buffer {
            _id: {
                let mut id = 0;
                unsafe { gl::GenBuffers(1, &mut id); }
                id
            },

            _stride: 0,
            _kind: 0,
            _draw: 0,

            _size: 0,
            _kind_size: 0,
            _length: 0,
        }
    }

    pub fn id(&self) -> GLuint { self._id }

    pub fn stride(&self) -> usize { self._stride }
    pub fn kind(&self) -> GLenum { self._kind }
    pub fn draw(&self) -> GLenum { self._draw }

    pub fn size(&self) -> usize { self._size }
    pub fn kind_size(&self) -> usize { self._kind_size }
    pub fn length(&self) -> usize { self._length }

    pub fn set<T>(&mut self, kind: GLenum, array: &[T], stride: usize, draw: GLenum) -> &mut Self {
        let length = array.len();
        let kind_size = mem::size_of::<T>();
        let size = kind_size * length;

        unsafe {
            gl::BindBuffer(kind, self._id);
    		gl::BufferData(kind, size as GLsizeiptr, mem::transmute(array.as_ptr()), draw);
    		gl::BindBuffer(kind, 0);
        };

        self._stride = stride;
        self._kind = kind;
        self._draw = draw;

        self._size = size;
        self._kind_size = kind_size;
        self._length = length;

        self
    }
}
