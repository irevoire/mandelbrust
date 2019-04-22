use crate::mandel;
use minifb::{Key, KeyRepeat};

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
            .update_with_buffer(&self.buffer)
            .unwrap_or_else(|e| log::error!("Window update failed: {}", e));
    }

    /// Update the mandel struct with the fetched event
    /// The user want to exit if this function return false
    pub fn handle_event(&mut self, mandel: &mut mandel::Mandel) -> bool {
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
        }
        update
    }

    fn handle_event_key(&self, mandel: &mut mandel::Mandel) -> bool {
        let mut update = false;
        self.window.get_keys_pressed(KeyRepeat::Yes).map(|keys| {
            for t in keys {
                match t {
                    Key::W => {
                        mandel.pos.y += (mandel.pos.y / mandel.zoom) * 10.0;
                        update = true;
                    }
                    Key::S => {
                        mandel.pos.y -= (mandel.pos.y / mandel.zoom) * 10.0;
                        update = true;
                    }
                    Key::A => {
                        mandel.pos.x += (mandel.pos.x / mandel.zoom) * 100.0;
                        update = true;
                    }
                    Key::D => {
                        mandel.pos.x -= (mandel.pos.x / mandel.zoom) * 100.0;
                        update = true;
                    }
                    Key::Space => {
                        mandel.zoom += mandel.zoom / 10.0;
                        update = true;
                    }
                    Key::X => {
                        mandel.zoom -= mandel.zoom / 10.0;
                        update = true;
                    }
                    Key::I => {
                        mandel.iter += mandel.iter / 10;
                        update = true;
                    }
                    Key::U => {
                        mandel.iter -= mandel.iter / 10;
                        update = true;
                    }
                    _ => (),
                }
            }
        });
        update
    }

    /// return the width of the window
    pub fn width(&self) -> usize {
        self.width
    }

    /// return the height of the window
    pub fn height(&self) -> usize {
        self.height
    }

    /// return the size of the window -> (width, height)
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
