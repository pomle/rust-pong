extern crate glium;

use std::path::Path;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::fs::File;

use pong::Player;
use glium::DisplayBuild;

fn getShader(path: &str) -> String {
  let mut file = match File::open(&path) {
      Err(why) => panic!("couldn't open {}: {}", path,
                                                 Error::description(&why)),
      Ok(file) => file,
  };
  let mut s = String::new();
  match file.read_to_string(&mut s) {
      Err(why) => panic!("couldn't read {}: {}", path,
                                                 Error::description(&why)),
      Ok(_) => print!("{} contains:\n{}", path, s),
  }
  s
}

pub struct Program {

}

pub struct Renderer {
  pub display: glium::backend::glutin_backend::GlutinFacade,
  color: String,
}

impl Renderer {
  pub fn new() -> Renderer {
    Renderer {
      display: glium::glutin::WindowBuilder::new().build_glium().unwrap(),
    }
  }

  pub fn renderEntity(entity: RenderEntity) {

  }
}

pub struct RenderEntity {
  pub program: glium::Program,
}

impl RenderEntity {
  pub fn new(display: glium::backend::glutin_backend::GlutinFacade,
             fragmentShader: String,
             vertexShader: String) -> RenderEntity {
    RenderEntity {
      program: glium::Program::from_source(&display,
                                           fragmentShader,
                                           vertexShader, None).unwrap();,
    }
  }
}
