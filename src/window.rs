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

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
