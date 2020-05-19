use mandelbrust::Mandel;
use minifb::{Key, KeyRepeat};
use std::{thread, time};

pub struct Window {
    window: minifb::Window,
    width: usize,
    height: usize,
    pub buffer: Vec<u32>, // color
}

impl Window {
    pub fn new(width: usize, height: usize) -> Result<Self, String> {
        let window = minifb::Window::new(
            "Mandelbrot",
            width,
            height,
            minifb::WindowOptions {
                resize: false, // TODO allow resize
                scale: minifb::Scale::X1,
                ..minifb::WindowOptions::default()
            },
        );
        if let Err(e) = window {
            return Err(format!("Unable to create window {}", e));
        };

        Ok(Window {
            // if the window creation fail we exit everything
            window: window.unwrap(),
            width,
            height,
            buffer: vec![0; width * height],
        })
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap_or_else(|e| eprintln!("Window update failed: {}", e));
    }

    /// Update the mandel struct with the fetched event
    /// The user want to exit if this function return false
    pub fn handle_event(&mut self, mandel: &mut Mandel) -> bool {
        let mut update = false;

        while !update {
            self.window.update(); // needed in order to fetch the new events

            if !self.window.is_open() {
                return false;
            }
            if self.window.is_key_down(Key::Escape) {
                return false;
            }

            update |= self.handle_event_key(mandel);
            thread::sleep(time::Duration::from_millis(10));
        }
        update
    }

    fn handle_event_key(&self, mandel: &mut Mandel) -> bool {
        let mut update = false;
        if let Some(keys) = self.window.get_keys_pressed(KeyRepeat::Yes) {
            for t in keys {
                match t {
                    Key::W | Key::Z | Key::Up => {
                        mandel.pos.y -= 100.0 / mandel.zoom;
                        update = true;
                    }
                    Key::S | Key::Down => {
                        mandel.pos.y += 100.0 / mandel.zoom;
                        update = true;
                    }
                    Key::A | Key::Q | Key::Left => {
                        mandel.pos.x -= 100.0 / mandel.zoom;
                        update = true;
                    }
                    Key::D | Key::Right => {
                        mandel.pos.x += 100.0 / mandel.zoom;
                        update = true;
                    }
                    Key::Space => {
                        mandel.pos.x += self.width as f64 * 0.25 / mandel.zoom;
                        mandel.pos.y += self.height as f64 * 0.25 / mandel.zoom;
                        mandel.zoom *= 2.0;
                        update = true;
                    }
                    Key::X => {
                        mandel.zoom /= 2.0;
                        mandel.pos.x -= self.width as f64 * 0.25 / mandel.zoom;
                        mandel.pos.y -= self.height as f64 * 0.25 / mandel.zoom;
                        update = true;
                    }
                    Key::I => {
                        mandel.iter += 3;
                        update = true;
                    }
                    Key::U => {
                        mandel.iter -= 3;
                        update = true;
                    }
                    _ => (),
                }
            }
        };
        update
    }

    /// return the dimensions of the window (width, height)
    pub fn dimension(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
