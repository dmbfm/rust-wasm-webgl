//! Module for low-level handling of WebGL shader programs

use { WebGLProgram, WebGLShader, GL };
use context::Context;
use std::rc::Rc;
use std::marker::PhantomData;
use std::option::Option;

/// Trait to wrap WebGL shader types.
pub trait ShaderTypeProvider {
    fn shader_type() -> u32;
}

/// Dummy type for vertex shaders
pub struct VertexShaderType {}

/// Dummy type for fragment shaders
pub struct FragmentShaderType {}

impl ShaderTypeProvider for VertexShaderType {
    fn shader_type() -> u32 {
        GL::VERTEX_SHADER()
    }
}

impl ShaderTypeProvider for FragmentShaderType {
    fn shader_type() -> u32 {
        GL::FRAGMENT_SHADER()
    }
}

/// Represents a WebGL Shader of a given type.
pub struct Shader<T> {
    /// The WebGL shader object.
    gl_shader: WebGLShader,

    /// The shader's source code.
    shader_source: String,

    /// The associated rendering context.
    context: Rc<Context>,

    /// Type parameter placeholder.
    phantom: PhantomData<T>
}


impl <T: ShaderTypeProvider> Shader<T> {
    /// Creates and compiles a new shader from a given source.
    pub fn new(context: Rc<Context>, source: &str) -> Option<Shader<T>> {
        context
            .compile_shader(source, T::shader_type())
            .map(|gl_shader|
                Shader {
                    gl_shader,
                    shader_source: String::from(source),
                    context: Rc::clone(&context),
                    phantom: PhantomData
                }
            )
    }
}

/// Represents a vertex shader.
pub type VertexShader = Shader<VertexShaderType>;

/// Represents a fragment shader.
pub type FragmentShader = Shader<FragmentShaderType>;

/// Represents a WebGL shader program.
pub struct Program {
    gl_program: WebGLProgram,
    context: Rc<Context>,
    v_shader: VertexShader,
    f_shader: FragmentShader
}

impl Program {
    pub fn new(context: Rc<Context>, v_source: &str, f_source: &str) -> Option<Program> {
        let f_shader = FragmentShader::new(context.clone(), f_source);
        let v_shader = VertexShader::new(context.clone(), v_source);
        let gl_program = context.gl.create_program();

        f_shader
            .and_then(
                |f_shader|
                    v_shader.and_then(
                        |v_shader|
                            context.link_shader(&v_shader.gl_shader, &f_shader.gl_shader).map(
                                |gl_program|
                                    Program {
                                        gl_program,
                                        context: context.clone(),
                                        v_shader,
                                        f_shader
                                    }
                            )
                    )
            )
    }
}