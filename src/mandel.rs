use packed_simd::{f64x4, m32x4, u32x4, u8x4};
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

        window
            .par_chunks_mut(width)
            .enumerate()
            .for_each(|(index, val)| {
                let y = f64x4::splat(index as f64);
                val.iter_mut().enumerate().for_each(|(i, v)| *v = i as u32);
                val.chunks_exact_mut(4).for_each(|data| {
                    let mut v = u32x4::new(data[0], data[1], data[2], data[3]);
                    let x: f64x4 = unsafe {
                        f64x4::new(
                            v.extract_unchecked(0) as f64,
                            v.extract_unchecked(1) as f64,
                            v.extract_unchecked(2) as f64,
                            v.extract_unchecked(3) as f64,
                        )
                    };
                    v = u32x4::splat(0);
                    let c_r = x / f64x4::splat(self.zoom) + x1;
                    let c_i = y / f64x4::splat(self.zoom) + y1;
                    let mut z_r = f64x4::splat(0.0);
                    let mut z_i = f64x4::splat(0.0);
                    let mut i = 0;
                    let mut results = u8x4::splat(0);

                    loop {
                        let partial_res = z_r * z_r + z_i * z_i;
                        if i == self.iter {
                            break;
                        } else if results == u8x4::splat(1) {
                            break;
                        }
                        let mask = results.eq(u8x4::splat(0));
                        let mask = mask.select(
                            partial_res.gt(f64x4::splat(4.0)).into(),
                            m32x4::splat(false),
                        );

                        v = mask.select(u32x4::splat(i), v);
                        results = mask.select(u8x4::splat(i as u8), results);

                        let tmp = z_r;
                        z_r = z_r * z_r - z_i * z_i + c_r;
                        z_i = 2.0 * z_i * tmp + c_i;
                        i += 1;
                    }
                    data[0] = v.extract(0);
                    data[1] = v.extract(1);
                    data[2] = v.extract(2);
                    data[3] = v.extract(3);
                });
            });
    }
}
