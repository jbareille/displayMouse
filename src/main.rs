extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::cmp::max;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::MouseCursorEvent;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    mouse_pos: [f64; 2], // Mouse position.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(0.0,0.0).rot_deg(0.0);
            let length = 3.0;
            let width = 20.0;
            let x0 = self.mouse_pos[0] - width / 2.0;
            let x1 = self.mouse_pos[0] + width / 2.0;
            let y0 = self.mouse_pos[1] - length / 2.0;
            let y1 = self.mouse_pos[1] + length / 2.0;
            rectangle(RED, rectangle::rectangle_by_corners( x0, y0, x1, y1), transform, gl);

            let x0 = self.mouse_pos[0] - length / 2.0;
            let x1 = self.mouse_pos[0] + length / 2.0;
            let y0 = self.mouse_pos[1] - width / 2.0;
            let y1 = self.mouse_pos[1] + width / 2.0;
            rectangle(RED, rectangle::rectangle_by_corners( x0, y0, x1, y1), transform, gl);
        });
    }

    fn update_mouse(&mut self, mouse_pos: [f64; 2]) {
        self.mouse_pos = mouse_pos.clone();
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;


    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        mouse_pos: [0.0, 0.0],
    };


    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(mouse_pos) = e.mouse_cursor_args() {
            //println!("mouse_pos: {:?}", mouse_pos);
            app.update_mouse(mouse_pos);
        }
    }
}