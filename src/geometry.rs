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
    pub context: Rc<Context>,
    pub layout: VertexLayout,
    pub array_buffer: ArrayBuffer,
    pub element_buffer: Option<ElementBuffer>,
    pub vertex_count: u32,

    pub gl_vao: WebGLVertexArrayObject
}

pub fn calculate_stride(layout: &VertexLayout) -> u32 {
    layout.iter().fold(0, |acc, size| acc + 4 * size)
}

impl Geometry {
    /// Creates a new Geometry
    pub fn new(
        context: Rc<Context>,
        layout: VertexLayout,
        array_buffer: ArrayBuffer,
        element_buffer: Option<ElementBuffer>
    ) -> Geometry {
        let gl_vao = context.create_vertex_array();
        context.bind_vertex_array(&gl_vao);

        let stride = calculate_stride(&layout);
        let vertex_count = ((array_buffer.data.len() as u32) * 4) / stride;

        let mut offset: u32 = 0;
        for i in 0..layout.len() {
            context.gl.enable_vertex_attrib_array(i as u32);
            context.gl.vertex_attrib_pointer(i as u32, layout[i], GL::FLOAT(), false, if layout.len() == 1 { 0 } else { stride }, offset);
            offset = offset + layout[i] * 4;
        }

        Geometry {
            context: context.clone(),
            layout,
            array_buffer,
            element_buffer,
            vertex_count,
            gl_vao
        }
    }

    /// Binds the geometry for rendering
    pub fn bind(self: &Geometry) {
        self.context.bind_vertex_array(&self.gl_vao);
    }
}