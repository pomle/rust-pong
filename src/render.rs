extern crate glium;

use glium::DisplayBuild;

pub struct Renderer {
  pub display: glium::backend::Facade,

}

impl Renderer {
  pub fn new() -> Renderer {
    Renderer {
      display: glium::glutin::WindowBuilder::new().build_glium().unwrap(),
    }
  }
}
