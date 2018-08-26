use { WebGLProgram, WebGLShader, GL };

use context::Context;

use std::rc::Rc;
use std::marker::PhantomData;
use std::option::Option;

/// Trait to wrap WebGL shader types.
pub trait ShaderTypeProvider {
    fn shader_type() -> u32;
}

/// Represents a WebGL Shader of a given type.
pub struct Shader<T> {
    gl_shader: WebGLShader,
    shader_source: String,
    context: Rc<Context>,

    phantom: PhantomData<T>
}

/// Dummy type for vertex shaders
pub struct VertexShader {}

/// Dummy type for fragment shaders
pub struct FragmentShader {}

impl ShaderTypeProvider for VertexShader {
    fn shader_type() -> u32 {
        GL::VERTEX_SHADER()
    }
}

impl ShaderTypeProvider for FragmentShader {
    fn shader_type() -> u32 {
        GL::FRAGMENT_SHADER()
    }
}

/// Represents a WebGL shader program
struct Program {
    gl_program: WebGLProgram,
    v_shader: Shader<VertexShader>,
    f_shader: Shader<FragmentShader>
}


impl <T: ShaderTypeProvider> Shader<T> {
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
