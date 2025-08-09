extern crate piston_window;

use piston_window::*;
use std::time::Instant;

pub struct App {
    mouse_pos: [f64; 2],  // Mouse position.
    last_update: Instant, // Dernière mise à jour.
    tick: u64,
    mouse: u64,
    fps: u8,
    mps: u8, // Mouse position updates per second.
}

impl App {
    fn render(&mut self, glyphs: &mut Glyphs, window: &mut PistonWindow, e: Event) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        window.draw_2d(&e, |c, g, d| {

            clear([1.0, 1.0, 1.0, 1.0], g);

            let transform = c.transform.trans(0.0, 0.0).rot_deg(0.0);
            let length = 3.0;
            let width = 20.0;
            let x0 = self.mouse_pos[0] - width / 2.0;
            let x1 = self.mouse_pos[0] + width / 2.0;
            let y0 = self.mouse_pos[1] - length / 2.0;
            let y1 = self.mouse_pos[1] + length / 2.0;
            rectangle(
                RED,
                rectangle::rectangle_by_corners(x0, y0, x1, y1),
                transform,
                g,
            );

            let x0 = self.mouse_pos[0] - length / 2.0;
            let x1 = self.mouse_pos[0] + length / 2.0;
            let y0 = self.mouse_pos[1] - width / 2.0;
            let y1 = self.mouse_pos[1] + width / 2.0;
            rectangle(
                GREEN,
                rectangle::rectangle_by_corners(x0, y0, x1, y1),
                transform,
                g,
            );

            // Afficher les FPS.
            let fps_text = format!("FPS: {:.2}", self.fps);

            let txt = fps_text.as_str();

            Text::new_color([0.0, 0.0, 0.0, 1.0], 20)
                .draw(&txt, glyphs, &c.draw_state, transform.trans(10.0, 100.0), g)
                .unwrap();

            // Afficher les MPS.
            let mps_text = format!("MPS: {:.2}", self.mps);
            let txt = mps_text.as_str();

            Text::new_color([0.0, 0.0, 0.0, 1.0], 20)
                .draw(&txt, glyphs, &c.draw_state, transform.trans(10.0, 200.0), g)
                .unwrap();

            glyphs.factory.encoder.flush(d);
        });
    }

    fn update_fps(&mut self) {
        let now = Instant::now();
        let duration = now.duration_since(self.last_update);

        if duration.as_millis() > 1000 {
            self.last_update = now;
            self.fps = (self.tick * 1000 / duration.as_millis() as u64) as u8;
            // If more than a second has passed, reset the tick.
            self.tick = 0;

            self.mps = (self.mouse * 1000 / duration.as_millis() as u64) as u8;
            self.mouse = 0;
        } else {
            // Increment the tick.
            self.tick += 1;
        }
    }

    fn update_mouse(&mut self, mouse_pos: [f64; 2]) {
        if mouse_pos[0] != self.mouse_pos[0] || mouse_pos[1] != self.mouse_pos[1] {
            self.mouse += 1;
        }
        self.mouse_pos = mouse_pos.clone();
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new("displayMouse", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(true)
        .transparent(true)
        .build()
        .unwrap();

    let mut app = App {
        mouse_pos: [0.0, 0.0],
        last_update: Instant::now(),
        tick: 0,
        fps: 0,
        mouse: 0,
        mps: 0,
    };

    let mut glyphs = window.load_font("assets/FiraSans-Regular.ttf").unwrap();
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        let current_event = e.clone();
        if let Some(_args) = e.render_args() {
            app.render(&mut glyphs, &mut window, current_event);
        }

        if let Some(mouse_pos) = e.mouse_cursor_args() {
            app.update_mouse(mouse_pos);
        }

        app.update_fps();
    }
}
