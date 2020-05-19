use rayon::prelude::*;

pub struct Pos {
    pub x: f64,
    pub y: f64,
}

pub struct Mandel {
    pub pos: Pos,
    pub iter: u32,
    pub zoom: f64,
}

impl Mandel {
    pub fn new(x: f64, y: f64, iter: u32, zoom: f64) -> Self {
        Mandel {
            pos: Pos { x, y },
            iter,
            zoom,
        }
    }

    pub fn compute(&self, window: &mut [u32], width: usize, _height: usize) {
        let x1 = self.pos.x;
        let y1 = self.pos.y;

        let mut chunks: Vec<&mut [u32]> = window.chunks_mut(width).collect();
        chunks.par_iter_mut().enumerate().for_each(|(index, val)| {
            let y = index;
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

                val[x] = i;
            }
        });
    }
}
