#![feature(use_extern_macros)]
#![allow(dead_code)]

extern crate wasm_bindgen;

use std::rc::Rc;
use wasm_bindgen::prelude::*;
use context::Context;
use shader::{Shader, VertexShader, FragmentShader, Program};
use buffer::{Buffer, ArrayBuffer, ElementBuffer};
use geometry::{ Geometry, vertex_p3 };

pub mod shader;
pub mod context;
pub mod buffer;
pub mod geometry;
pub mod mesh;

// Misc javascript imports
#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(module = "./js/glue")]
    fn test(x: &[u32]);
}

#[wasm_bindgen]
extern {
    pub type XXX;

    #[wasm_bindgen(method, js_name = xxx)]
    pub fn xxx(this: &XXX );
}

// HTMLCanvas bindings
#[wasm_bindgen]
extern {
    pub type HTMLCanvasElement;

    #[wasm_bindgen(method, getter, structural)]
    pub fn width(this: &HTMLCanvasElement) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn height(this: &HTMLCanvasElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_width(this: &HTMLCanvasElement, width: u32) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_height(this: &HTMLCanvasElement, height: u32) -> u32;

    #[wasm_bindgen(module = "./js/glue")]
    fn _get_webgl_rendering_context(canvas: &HTMLCanvasElement) -> Option<WebGLRenderingContext>;

    #[wasm_bindgen(module = "./js/glue")]
    fn _get_2d_rendering_context(canvas: &HTMLCanvasElement) -> Option<Canvas2DRenderingContext>;
}

// Additional/custom HTMLCanvas methods
impl HTMLCanvasElement {
    pub fn get_webgl_rendering_context(self: &HTMLCanvasElement) -> Option<WebGLRenderingContext> {
        _get_webgl_rendering_context(self)
    }
}

// WebGLRenderingContext (i.e., webgl1) bindings
#[wasm_bindgen]
extern "C" {
    pub type WebGLRenderingContext;

    #[wasm_bindgen(method, js_name = createShader)]
    pub fn create_shader(this: &WebGLRenderingContext, shader_type: u32) -> WebGLShader;

    #[wasm_bindgen(method, js_name = shaderSource)]
    pub fn shader_source(this: &WebGLRenderingContext, shader: &WebGLShader, source: &str);

    #[wasm_bindgen(method, js_name = compileShader)]
    pub fn compile_shader(this: &WebGLRenderingContext, shader: &WebGLShader);

    #[wasm_bindgen(method, js_name = getShaderInfoLog)]
    pub fn get_shader_info_log(this: &WebGLRenderingContext, shader: &WebGLShader) -> String;

    #[wasm_bindgen(method, js_name = createProgram)]
    pub fn create_program(this: &WebGLRenderingContext) -> WebGLProgram;

    #[wasm_bindgen(method, js_name = attachShader)]
    pub fn attach_shader(this: &WebGLRenderingContext, program: &WebGLProgram, shader: &WebGLShader);

    #[wasm_bindgen(method, js_name = linkProgram)]
    pub fn link_program(this: &WebGLRenderingContext, program: &WebGLProgram);

    #[wasm_bindgen(method, js_name = useProgram)]
    pub fn use_program(this: &WebGLRenderingContext, program: &WebGLProgram);

    #[wasm_bindgen(method, js_name = getProgramParameter)]
    pub fn get_program_parameter_b(this: &WebGLRenderingContext, program: &WebGLProgram, p_name: u32) -> bool;

    #[wasm_bindgen(method, js_name = getProgramParameter)]
    pub fn get_program_parameter_i(this: &WebGLRenderingContext, program: &WebGLProgram, p_name: u32) -> i32;

    #[wasm_bindgen(method, js_name = createBuffer)]
    pub fn create_buffer(this: &WebGLRenderingContext) -> WebGLBuffer;

    #[wasm_bindgen(method, js_name = deleteBuffer)]
    pub fn delete_buffer(this: &WebGLRenderingContext, buffer: &WebGLBuffer);

    #[wasm_bindgen(method, js_name = bindBuffer)]
    pub fn bind_buffer(this: &WebGLRenderingContext, target: u32, buffer: &WebGLBuffer);

    #[wasm_bindgen(method, js_name = bufferData)]
    pub fn buffer_data_array_f32(this: &WebGLRenderingContext, target: u32, data: &[f32], usage: u32);

    #[wasm_bindgen(method, js_name = enableVertexAttribArray)]
    pub fn enable_vertex_attrib_array(this: &WebGLRenderingContext, index: u32);

    #[wasm_bindgen(method, js_name = clearColor)]
    pub fn clear_color(this: &WebGLRenderingContext, r: f32, g: f32, b: f32, a: f32);

    #[wasm_bindgen(method)]
    pub fn clear(this: &WebGLRenderingContext, mask: u32);

    #[wasm_bindgen(method, js_name = drawArrays)]
    pub fn draw_arrays(this: &WebGLRenderingContext, mode: u32, first: u32, count: u32);

    /// The WebGLRenderingContext.vertexAttribPointer() method of the WebGL API binds the buffer
    /// currently bound to gl.ARRAY_BUFFER to a generic vertex attribute of the current vertex
    /// buffer object and specifies its layout.
    ///
    /// void gl.vertexAttribPointer(index, size, type, normalized, stride, offset)
    #[wasm_bindgen(method, js_name = vertexAttribPointer)]
    pub fn vertex_attrib_pointer(this: &WebGLRenderingContext, index: u32, size: u32, data_type: u32, normalized: bool, stride: u32, offset: u32);

    // Extensions
    #[wasm_bindgen(module = "./js/glue")]
    pub fn _get_ext_oes_vertex_array_object(gl: &WebGLRenderingContext) -> Option<OES_vertex_array_object >;

    // Constants
    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn VERTEX_SHADER() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn FRAGMENT_SHADER() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn ARRAY_BUFFER() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn ELEMENT_ARRAY_BUFFER() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn STATIC_DRAW() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn DYNAMIC_DRAW() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn STREAM_DRAW() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn LINK_STATUS() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn BYTE() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn SHORT() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn UNSIGNED_BYTE() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn UNSIGNED_SHORT() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn FLOAT() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn COLOR_BUFFER_BIT() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn DEPTH_BUFFER_BIT() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn STENCIL_BUFFER_BIT() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn POINTS() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn LINE_STRIP() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn LINE_LOOP() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn LINES() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn TRIANGLE_STRIP() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn TRIANGLE_FAN() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn TRIANGLES() -> u32;

}

#[wasm_bindgen]
extern {
    pub type OES_vertex_array_object ;


    #[wasm_bindgen(module = "./js/glue")]
    pub fn _create_vertex_array_oes(oes: &OES_vertex_array_object) -> WebGLVertexArrayObject;

    #[wasm_bindgen(module = "./js/glue")]
    pub fn _bind_vertex_array_oes(oes: &OES_vertex_array_object, vao: &WebGLVertexArrayObject);

    #[wasm_bindgen(module = "./js/glue")]
    pub fn _delete_vertex_array_oes(oes: &OES_vertex_array_object, vao: WebGLVertexArrayObject);
}

/// Shorthand for WebGLRenderingContext.
pub type GL = WebGLRenderingContext;

/// Binding for associated WebGL types.
#[wasm_bindgen]
extern {
    pub type WebGLShader;
    pub type WebGLProgram;
    pub type WebGLBuffer;
    pub type WebGLVertexArrayObject;
}

/// Binding for canvas context.
#[wasm_bindgen]
extern {
    pub type Canvas2DRenderingContext;
}


// ----------> Prototyping area <------------ //

#[wasm_bindgen]
pub fn test_get_context(canvas: &HTMLCanvasElement, context_type: &str) {
    let triangle = vec![
        0.0, 0.0, 0.0,
        0.0, 0.5, 0.0,
        0.5, 0.5, 0.0
    ];

    let gl: WebGLRenderingContext = canvas.get_webgl_rendering_context().unwrap();
    let context = Context::new(gl);
    let context_rc = Rc::new(context);
    let shader: Option<VertexShader> = Shader::new(Rc::clone(&context_rc), "void main() { gl_Position = vec4(0.0); }");
    let buffer: Option<ArrayBuffer> = Buffer::new(Rc::clone(&context_rc), triangle);
    let program = Program::new(context_rc.clone(), "attribute vec3 pos; void main() { gl_Position = vec4(pos, 1.0); }", "void main() { gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0); }").unwrap();

    let triangle_geometry = Geometry::new(context_rc.clone(), vertex_p3(), buffer.unwrap(), None);

    let triangle_mesh = mesh::Mesh::new(context_rc.clone(), Rc::new(triangle_geometry), Rc::new(program));

    context_rc.clear();
    triangle_mesh.render();

//    match program {
//        Some(_) => log("Program compiled!"),
//        None => log("Program failed!")
//    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log("hello");
    test(&vec![1, 2, 3, 4]);
}

// ------------------------------------------ //
