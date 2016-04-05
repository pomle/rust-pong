#[macro_use]
extern crate glium;
extern crate time;

mod pong;
mod render;

#[test]
fn it_works() {
  let mut vec = pong::Vector2::new();
  vec.set(10.0, 5.0);
  assert_eq!(vec.getLength(), 11.18034);
  vec.setLength(8.0);
  assert_eq!(vec.x, 7.155418);
  assert_eq!(vec.y, 3.577709);
}

fn main() {
  use glium::DisplayBuild;
  use glium::Surface;
  use pong::Player;
  use std::vec::Vec;

  let mut players = vec![
    Player::new(),
    Player::new(),
  ];



  let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

  #[derive(Copy, Clone)]
  struct Vertex {
      position: [f32; 2],
  }

  implement_vertex!(Vertex, position);

  let shape = vec![
    Vertex { position: [-0.5, -0.5] },
    Vertex { position: [ 0.5,  0.5] },
    Vertex { position: [ 0.5, -0.5] },
  ];

  let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
  let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

  let vertex_shader_src = r#"
      #version 140
      uniform float t;
      in vec2 position;
      void main() {
          vec2 pos = position;
          pos.x += t;
          gl_Position = vec4(pos, 0.0, 1.0);
      }
  "#;

  let fragment_shader_src = r#"
      #version 140
      out vec4 color;
      void main() {
          color = vec4(1.0, 1.0, 1.0, 1.0);
      }
  "#;

  let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

  let mut dt: f64;
  let mut t: f64 = timestamp();
  let mut tl = t;
  let mut elapsed: f64 = 0.0;

  loop {
    t = timestamp();
    dt = t - tl;

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 1.0);
    target.draw(&vertex_buffer,
                &indices,
                &program,
                &uniform! {
                  t: elapsed.sin() as f32,
                },
                &Default::default()).unwrap();
    target.finish().unwrap();

    tl = t;
    elapsed += dt;
  }
}

fn timestamp() -> f64 {
  let timespec = time::get_time();
  let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
  mills
}

fn draw(mut target: glium::Frame) -> glium::Frame {
  use glium::Surface;
  target.clear_color(0.0, 0.0, 0.0, 1.0);
  target
}