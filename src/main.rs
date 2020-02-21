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
    size: f64,
    pos: [f64;2],
    vel: [f64;2]
}

pub struct Paddle {
    size: [f64;2],
    pos: [f64;2],
}

fn make_pad(pos: [f64;2]) -> Paddle {
    Paddle {
        size: [20.0, 80.0],
        pos,
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.

    ball: Ball,
    paddles: [Paddle;2],
    court: [f64;2],
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        dbg!(args);
        use graphics::rectangle;
        use graphics::clear;

        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const FOREGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let rects = [
            rectangle::square(self.ball.pos[0], self.ball.pos[1], self.ball.size),
            [self.paddles[0].pos[0], self.paddles[0].pos[1],
                self.paddles[0].size[0], self.paddles[0].size[1]],
            [self.paddles[1].pos[0], self.paddles[1].pos[1],
                self.paddles[1].size[0], self.paddles[1].size[1]],
        ];

        self.gl.draw(args.viewport(), |context, gl| {
            // Clear the screen.
            clear(BACKGROUND, gl);

            // Draw a box rotating around the middle of the screen.
            for rect in &rects {
                rectangle(FOREGROUND, *rect, context.transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.ball.pos[0] += self.ball.vel[0] * args.dt;
        self.ball.pos[1] += self.ball.vel[1] * args.dt;

        if
            (self.ball.vel[1] < 0.0 && self.ball.pos[1] < 0.0) ||
            (self.ball.vel[1] > 0.0 && self.ball.pos[1] > self.court[1] - self.ball.size) {
                self.ball.vel[1] *= -1.0;
        }
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

    let court_size = [600.0, 300.0];
    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        ball: Ball {
            size: 50.0,
            pos: [0.0, 0.0],
            vel: [100.0, 100.0],
        },
        paddles: [
            make_pad([20.0, court_size[1] / 2.0]),
            make_pad([court_size[0] - 20.0, court_size[1] / 2.0]),
        ],
        court: court_size,
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
