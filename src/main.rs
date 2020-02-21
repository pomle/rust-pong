extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct Ball {
    pos: [f64;2],
    vel: [f64;2]
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    ball: Ball,

}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::rectangle;
        use graphics::clear;

        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const FOREGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let ball = rectangle::square(self.ball.pos[0], self.ball.pos[1], 50.0);

        self.gl.draw(args.viewport(), |context, gl| {
            // Clear the screen.
            clear(BACKGROUND, gl);

            // Draw a box rotating around the middle of the screen.
            rectangle(FOREGROUND, ball, context.transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.ball.pos[0] += self.ball.vel[0];
        self.ball.pos[1] += self.ball.vel[1];
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [600, 300])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        ball: Ball {
            pos: [0.0, 0.0],
            vel: [1.0, 1.0],
        },
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
