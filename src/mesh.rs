//! Mesh

use geometry::Geometry;
use shader::Program;
use context::Context;
use GL;
use Rc;

pub struct Mesh {
    geometry: Rc<Geometry>,
    shader_program: Rc<Program>,
    context: Rc<Context>
}

// TODO: render
impl Mesh {
    pub fn new(context: Rc<Context>, geometry: Rc<Geometry>, shader_program: Rc<Program>) -> Mesh {
        Mesh {
            geometry: geometry.clone(),
            shader_program: shader_program.clone(),
            context: context.clone()
        }
    }

    pub fn render(self: &Mesh) {
        self.shader_program.use_program();
        self.geometry.bind();

        self.context.gl.draw_arrays(GL::TRIANGLES(), 0, self.geometry.vertex_count);
    }
}
