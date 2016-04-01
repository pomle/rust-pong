extern crate glium;

use std::vec::Vec;

const VERTEX_SHADER: &'static str = r#"
    uniform float t;
    in vec2 position;
    void main() {
        vec2 pos = position;
        pos.x += t;
        gl_Position = vec4(pos, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &'static str = r#"
    out vec4 color;
    void main() {
        color = vec4(1.0, 1.0, 1.0, 1.0);
    }
"#;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);


pub struct Vector2 {
  pub x: f32,
  pub y: f32,
}

impl Vector2 {
  fn new() -> Vector2 {
    Vector2 { x: 0.0, y: 0.0 }
  }
}

pub struct Ball {
  pub pos: Vector2,
}

impl Ball {
  pub fn new() -> Ball {
    Ball {
      pos: Vector2::new(),
    }
  }
}

pub struct Player {
  vertex_buffer: glium::VertexBuffer<Vertex>,
  indices: glium::index::NoIndices,
  program: glium::Program,
  time: f64,
  pub pos: Vector2,
}

impl Player {
  pub fn new(display: glium::DisplayBuild) -> Player {
    let shape = vec![
      Vertex { position: [-0.5, -0.5] },
      Vertex { position: [ 0.5,  0.5] },
      Vertex { position: [ 0.5, -0.5] },
      Vertex { position: [-0.5,  0.5] },
    ];

    Player {
      vertex_buffer: glium::VertexBuffer::new(&display, &shape).unwrap(),
      indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
      time: 0.0,
      pos: Vector2::new(),
    }
  }

  pub fn setup(&self, display: glium::VertexBuffer<Vertex>) {
    let shape = vec![
      Vertex { position: [-0.5, -0.5] },
      Vertex { position: [ 0.5,  0.5] },
      Vertex { position: [ 0.5, -0.5] },
      Vertex { position: [-0.5,  0.5] },
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
  }

  pub fn draw(&self, mut frame: glium::Frame) -> glium::Frame {
    use glium::Surface;
    frame.draw(&self.vertex_buffer,
      &self.indices,
      &self.program,
      &uniform! {
        t: self.time as f32,
      },
      &Default::default())
      .unwrap();
    frame
  }

  pub fn update(&self, &dt: f64) {
    self.time += dt;
  }
}
