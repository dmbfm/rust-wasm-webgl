use { WebGLRenderingContext, WebGLShader };

pub struct Context {
    pub gl: WebGLRenderingContext
}

impl Context {
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
}