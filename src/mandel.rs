use faster::*;
use packed_simd::f64x4;
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
        let x1 = f64x4::splat(self.pos.x);
        let y1 = f64x4::splat(self.pos.y);

        let mut chunks: Vec<&mut [u32]> = window.chunks_mut(width).collect();
        chunks.par_iter_mut().enumerate().for_each(|(index, val)| {
            let y = f64x4::splat(index as f64);
            val.iter_mut().enumerate().for_each(|(i, v)| *v = i as u32);
            val.simd_iter_mut(u32s(0)).simd_for_each(|v| {
                let x = f64x4::new(
                    v.extract(0) as f64,
                    v.extract(1) as f64,
                    v.extract(2) as f64,
                    v.extract(3) as f64,
                );
                *v = u32s(0);
                let c_r = x / f64x4::splat(self.zoom) + x1;
                let c_i = y / f64x4::splat(self.zoom) + y1;
                let mut z_r = f64x4::splat(0.0);
                let mut z_i = f64x4::splat(0.0);
                let mut i = 0;
                let mut results = u8s(0);

                loop {
                    let partial_res = z_r * z_r + z_i * z_i;
                    if i == self.iter {
                        break;
                    } else if results == u8s(1) {
                        break;
                    }
                    for index in 0..4 {
                        if results.extract(index) == 0 && partial_res.extract(index) > 4.0 {
                            *v = v.replace(index, i);
                            results = results.replace(index, 1);
                        }
                    }
                    let tmp = z_r;
                    z_r = z_r * z_r - z_i * z_i + c_r;
                    z_i = 2.0 * z_i * tmp + c_i;
                    i += 1;
                }
            });
        });
    }
}
