#[macro_use]
extern crate glium;

fn main() {
  use std::time::Duration;
  use glium::DisplayBuild;
  use glium::Surface;


  let wait = Duration::from_millis(16);

  let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

  loop {
    let mut target = display.draw();
    target = draw(target);
    target.finish().unwrap();
    std::thread::sleep(wait);
  }
}

fn draw(mut target: glium::Frame) -> glium::Frame {
  use glium::Surface;
  target.clear_color(0.0, 0.0, 0.0, 1.0);
  target
}