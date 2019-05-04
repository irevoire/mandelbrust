use crate::window;
use std::time::Instant;

pub struct Pos {
    pub x: f64,
    pub y: f64,
}

pub struct Mandel {
    pub pos: Pos,
    pub iter: usize,
    pub zoom: f64,
}

impl Mandel {
    pub fn new(x: f64, y: f64, iter: usize, zoom: f64) -> Self {
        Mandel {
            pos: Pos { x, y },
            iter,
            zoom,
        }
    }

    pub fn compute(&self, window: &mut window::Window) {
        let width = window.width();
        let height = window.height();

        let x1 = self.pos.x;
        let y1 = self.pos.y;

        let now = Instant::now();

        for y in 0..height {
            for x in 0..width {
                let c_r = x as f64 / self.zoom + x1;
                let c_i = y as f64 / self.zoom + y1;
                let mut z_r = 0.0;
                let mut z_i = 0.0;
                let mut i = 0;

                while (z_r * z_r + z_i * z_i < 4.0) && i < self.iter {
                    let tmp = z_r;
                    z_r = z_r * z_r - z_i * z_i + c_r;
                    z_i = 2.0 * z_i * tmp + c_i;
                    i += 1;
                }

                if i == self.iter {
                    window.buffer[x + y * width] = 0x00000000;
                } else {
                    window.buffer[x + y * width] = hue_to_rgb(
                        i as f32 * (360.0 / self.iter as f32),
                        1.0,
                        i as f32 / self.iter as f32,
                    );
                }
            }
        }

        let duration = now.elapsed();
        println!(
            "{:.5} seconds for whatever you did.",
            duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
        );
    }
}

fn hue_to_rgb(hue: f32, saturation: f32, value: f32) -> u32 {
    let c: f32 = saturation * value;
    let x: f32 = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs()) as f32;
    let m: f32 = value - c;
    let (r, g, b) = match hue as u32 {
        0...60 => (c, x, 0.0),
        60...119 => (x, c, 0.0),
        120...179 => (0.0, c, x),
        180...239 => (0.0, x, c),
        240...299 => (x, 0.0, c),
        300...359 => (c, 0.0, x),
        _ => return 0,
    };
    let (r, g, b) = ((r + m) * 255.0, (g + m) * 255.0, (b + m) * 255.0);
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}
