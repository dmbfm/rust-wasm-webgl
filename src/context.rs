//! Module for handling the global rendering context.

use {
    WebGLRenderingContext,
    WebGLShader,
    WebGLProgram,
    GL,
    OES_vertex_array_object ,
    WebGLVertexArrayObject,
    _get_ext_oes_vertex_array_object,
    _create_vertex_array_oes,
    _delete_vertex_array_oes,
    _bind_vertex_array_oes
};

/// Represents the current rendering context. @TODO: Add version and extension support (capabilities?)
pub struct Context {
    pub gl: WebGLRenderingContext,
    pub oes_vertex_array_object: OES_vertex_array_object
}

impl Context {
    pub fn new(gl: WebGLRenderingContext) -> Context {
        let oes_vertex_array_object = _get_ext_oes_vertex_array_object(&gl).unwrap();

        Context {
            gl,
            oes_vertex_array_object
        }
    }

    /// Creates a vertex array object.
    pub fn create_vertex_array(self: &Context) -> WebGLVertexArrayObject {
        _create_vertex_array_oes(&self.oes_vertex_array_object)
    }

    /// Deletes a vertex array object.
    pub fn delete_vertex_array(self: &Context, vao: WebGLVertexArrayObject) {
        _delete_vertex_array_oes(&self.oes_vertex_array_object, vao)
    }

    /// Binds a vertex array object.
    pub fn bind_vertex_array(self: &Context, vao: &WebGLVertexArrayObject) {
        _bind_vertex_array_oes(&self.oes_vertex_array_object, vao)
    }

    /// Compiles a shader of a given type.
    pub fn compile_shader(self: &Context, source: &str, shader_type: u32) -> Option<WebGLShader> {
        let shader: WebGLShader = self.gl.create_shader(shader_type);
        self.gl.shader_source(&shader, source);
        self.gl.compile_shader(&shader);

        let log: String = self.gl.get_shader_info_log(&shader);
        
        if log.len() > 0 {
            None
        } else {
            Some(shader)
        }
    }

    /// Links a shader program.
    pub fn link_shader(self: &Context, v_shader: &WebGLShader, f_shader: &WebGLShader) -> Option<WebGLProgram> {
        let program = self.gl.create_program();

        self.gl.attach_shader(&program, &v_shader);
        self.gl.attach_shader(&program, &f_shader);

        self.gl.link_program(&program);

        if self.gl.get_program_parameter_b(&program, GL::LINK_STATUS()) {
            Some(program)
        } else {
            None
        }
    }
}