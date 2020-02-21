#[macro_use]
extern crate glium;

fn main() {
    use glium::DisplayBuild;
    use glium::Surface;

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    implement_vertex!(Vertex, position);

    let shape = vec![
        Vertex {
            position: [-0.5, -0.5],
        },
        Vertex {
            position: [0.5, 0.5],
        },
        Vertex {
            position: [0.5, -0.5],
        },
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
          color = vec4(1.0, 0.0, 0.0, 1.0);
      }
  "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let start = std::time::Instant::now();
    let mut dt: f64;
    let mut t: f64 = timestamp(start);
    let mut tl = t;
    let mut elapsed: f64 = 0.0;

    loop {
        t = timestamp(start);
        dt = t - tl;

        let mut target = display.draw();
        target = draw(target);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniform! {
                  t: elapsed.sin() as f32,
                },
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        tl = t;
        elapsed += dt;
    }
}

fn timestamp(start: std::time::Instant) -> f64 {
    let timespec = std::time::Instant::now().duration_since(start);
    timespec.as_secs_f64()
}

fn draw(mut target: glium::Frame) -> glium::Frame {
    use glium::Surface;
    target.clear_color(0.0, 0.0, 0.0, 1.0);
    target
}
