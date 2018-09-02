//! Module for low-level webgl buffer handling.

use { WebGLBuffer, GL, WebGLRenderingContext };
use context::Context;
use std::marker::PhantomData;
use std::rc::Rc;

/// Trait to wrap webgl buffer types.
pub trait BufferTypeProvider {
    fn buffer_type() -> u32;
}

/// Dummy type for array buffers.
pub struct BufferTypeArrayBuffer {}

/// Dummy type for element array buffers.
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

/// Trait to handle uploading of buffer data from slices of different number types.
pub trait BufferArrayProvider<T> {
    fn buffer_data_array(context: Rc<Context>, buffer_type: u32, data: &[T], usage: u32);
}

impl BufferArrayProvider<f32> for f32 {
    fn buffer_data_array(context: Rc<Context>, buffer_type: u32, data: &[f32], usage: u32) {
        context.gl.buffer_data_array_f32(buffer_type, data, usage);
    }
}

/// Represents a WebGL buffer of a given type (T) and of a given underlying number type (F).
pub struct Buffer<T, F> {
    pub gl_buffer: WebGLBuffer,
    pub context: Rc<Context>,
    pub data: Vec<F>,

    phantom: PhantomData<T>
}

impl <T: BufferTypeProvider, F: BufferArrayProvider<F>> Buffer<T, F> {
    /// Creates a new Buffer from a given data, and uploads it to the webgl context.
    pub fn new(context: Rc<Context>, data: Vec<F>) -> Option<Buffer<T, F>> {
        let gl_buffer = context.gl.create_buffer();
        context.gl.bind_buffer(T::buffer_type(), &gl_buffer);
        F::buffer_data_array(context.clone(), T::buffer_type(), &data as &[F], GL::STATIC_DRAW());
        Some(Buffer {
                context: context.clone(),
                gl_buffer,
                data,
                phantom: PhantomData
            })
    }
}

impl<T, F> Drop for Buffer<T, F> {
    fn drop(&mut self) {
        self.context.gl.delete_buffer(&self.gl_buffer);
    }
}

/// Type for array buffers.
pub type ArrayBuffer = Buffer<BufferTypeArrayBuffer, f32>;

/// Type for element array buffers.
pub type ElementBuffer = Buffer<BufferTypeElementArrayBuffer, u32>;
