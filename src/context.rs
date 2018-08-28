//! Module for handling the global rendering context.

use { WebGLRenderingContext, WebGLShader, WebGLProgram, GL };

pub enum WebGLContextVersion {
    WebGL1,
    WebGL2
}

/// Represents the current rendering context. @TODO: Add version and extension support (capabilities?)
pub struct Context {
    pub gl: WebGLRenderingContext,
    // pub gl_version: WebGLContextVersion

}

impl Context {
    pub fn new(gl: WebGLRenderingContext) -> Context {
        Context { gl }
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