use mandelbrust::Mandel;
use minifb::{Key, KeyRepeat};
use std::time;

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
                resize: true,
                scale: minifb::Scale::X1,
                ..minifb::WindowOptions::default()
            },
        );
        if let Err(e) = window {
            return Err(format!("Unable to create window {}", e));
        };
        let mut window = window.unwrap();
        window.limit_update_rate(Some(time::Duration::from_secs(1) / 30));

        Ok(Window {
            // if the window creation fail we exit everything
            window,
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
        loop {
            self.window.update(); // needed in order to fetch the new events

            if !self.window.is_open() {
                return false;
            }

            let (width, height) = self.window.get_size();
            if width != self.width || height != self.height {
                self.width = width;
                self.height = height;
                self.buffer.resize(self.width * self.height, 0);
                return true;
            }

            if let Some(b) = self.handle_event_key(mandel) {
                return b;
            }
        }
    }

    fn handle_event_key(&self, mandel: &mut Mandel) -> Option<bool> {
        if let Some(keys) = self.window.get_keys_pressed(KeyRepeat::Yes) {
            for k in &keys {
                match k {
                    Key::Escape => return Some(false),
                    Key::W | Key::Z | Key::Up => {
                        mandel.pos.y -= 100.0 / mandel.zoom;
                    }
                    Key::S | Key::Down => {
                        mandel.pos.y += 100.0 / mandel.zoom;
                    }
                    Key::A | Key::Q | Key::Left => {
                        mandel.pos.x -= 100.0 / mandel.zoom;
                    }
                    Key::D | Key::Right => {
                        mandel.pos.x += 100.0 / mandel.zoom;
                    }
                    Key::Space => {
                        mandel.pos.x += self.width as f64 * 0.25 / mandel.zoom;
                        mandel.pos.y += self.height as f64 * 0.25 / mandel.zoom;
                        mandel.zoom *= 2.0;
                    }
                    Key::X => {
                        mandel.zoom /= 2.0;
                        mandel.pos.x -= self.width as f64 * 0.25 / mandel.zoom;
                        mandel.pos.y -= self.height as f64 * 0.25 / mandel.zoom;
                    }
                    Key::I => {
                        mandel.iter += 3;
                    }
                    Key::U => {
                        mandel.iter -= 3;
                    }
                    _ => (),
                }
            }
            if keys.is_empty() {
                return None;
            }
        };
        Some(true)
    }

    /// return the dimensions of the window (width, height)
    pub fn dimension(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
