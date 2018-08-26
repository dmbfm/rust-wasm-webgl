use super::{ WebGLBuffer, GL, WebGLRenderingContext };
use context::Context;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct Buffer<T, F> {
    gl_buffer: WebGLBuffer,
    data: Vec<F>,
    phantom: PhantomData<T>
}

trait BufferTypeProvider {
    fn buffer_type() -> u32;
}

pub struct BufferTypeArrayBuffer {}

pub struct BufferTypeElementArrayBuffer {}

impl BufferTypeProvider for BufferTypeArrayBuffer {
    fn buffer_type () -> u32 {
        GL::ARRAY_BUFFER()
    }
}

impl BufferTypeProvider for BufferTypeElementArrayBuffer {
    fn buffer_type () -> u32 {
        GL::ELEMENT_ARRAY_BUFFER()
    }
}

pub type ArrayBuffer = Buffer<BufferTypeArrayBuffer, f32>;
pub type ElementBuffer = Buffer<BufferTypeElementArrayBuffer, u32>;

impl <T: BufferTypeProvider, F> Buffer<T, F> {
    pub fn new(context: Rc<Context>, data: Vec<F>) -> Option<Buffer<T, F>> {
        let gl_buffer = context.gl.create_buffer();
        context.gl.bind_buffer(T::buffer_type(), &gl_buffer);
        Some(Buffer {
                gl_buffer,
                data,
                phantom: PhantomData
            })
    }
}