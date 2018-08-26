#![feature(use_extern_macros)]
#![allow(dead_code)]

mod shader;
mod context;
mod buffer;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use context::Context;

use shader::{Shader, VertexShader, FragmentShader};

use buffer::{Buffer, ArrayBuffer, ElementBuffer};

use std::rc::Rc;

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
    fn _get_2d_rendering_context(canvas: &HTMLCanvasElement) -> Option<Canvas2DRenderingContext>;
}

impl HTMLCanvasElement {
    pub fn get_webgl_rendering_context(self: &HTMLCanvasElement) -> Option<WebGLRenderingContext> {
        _get_webgl_rendering_context(self)
    }
}

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

    #[wasm_bindgen(method, js_name = createBuffer)]
    pub fn create_buffer(this: &WebGLRenderingContext) -> WebGLBuffer;

    #[wasm_bindgen(method, js_name = bindBuffer)]
    pub fn bind_buffer(this: &WebGLRenderingContext, target: u32, buffer: &WebGLBuffer);

    // Constants
    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn VERTEX_SHADER() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn FRAGMENT_SHADER() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn ARRAY_BUFFER() -> u32;

    #[wasm_bindgen(static_method_of = WebGLRenderingContext, getter, structural)]
    pub fn ELEMENT_ARRAY_BUFFER() -> u32;
}

pub type GL = WebGLRenderingContext;

#[wasm_bindgen]
extern {
    pub type WebGLShader;
    pub type WebGLBuffer;
}

#[wasm_bindgen]
extern {
    pub type WebGLProgram;
}


#[wasm_bindgen]
extern {
    pub type Canvas2DRenderingContext;
}


#[wasm_bindgen]
pub fn test_get_context(canvas: &HTMLCanvasElement, context_type: &str) {
    let gl: WebGLRenderingContext = canvas.get_webgl_rendering_context().unwrap();
    let context = Context { gl };
    let context_rc = Rc::new(context);
    let shader: Option<Shader<VertexShader>> = Shader::new(Rc::clone(&context_rc), "void main() { gl_Position = vec4(0.0); }");
    let buffer: Option<ArrayBuffer> = Buffer::new(Rc::clone(&context_rc), vec![1.0]);

    match shader {
        Some(_) => log("Shader compiled!"),
        None => log("shader failed!")
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log("hello");
    test(&vec![1, 2, 3, 4]);
}


// Shader code

