//! Geometry

use { GL, WebGLVertexArrayObject };
use context::Context;
use buffer::{ ArrayBuffer, ElementBuffer };
use std::rc::Rc;

type VertexLayout = Vec<u32>;

pub fn vertex_p3() -> VertexLayout {
    vec![3]
}

pub fn vertex_p3n3() -> VertexLayout {
    vec![3, 3]
}

pub fn vertex_p3n3uv2() -> VertexLayout {
    vec![3, 3, 2]
}

pub fn vertex_p4n3uv2() -> VertexLayout {
    vec![4, 3, 2]
}

pub struct Geometry {
    context: Rc<Context>,
    layout: VertexLayout,
    array_buffer: ArrayBuffer,
    element_buffer: Option<ElementBuffer>,

    gl_vao: WebGLVertexArrayObject
}

impl Geometry {
    pub fn new(
        context: Rc<Context>,
        layout: VertexLayout,
        array_buffer: ArrayBuffer,
        element_buffer: Option<ElementBuffer>
    ) -> Geometry {
        let gl_vao = context.gl.create_vertex_array();

        Geometry {
            context: context.clone(),
            layout,
            array_buffer,
            element_buffer,
            gl_vao
        }
    }
}