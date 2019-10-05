use rayon::prelude::*;

pub fn compute(update: &fractal::Update, window: &mut fractal::Window) {
    let width = window.width();

    let x1 = update.coord.x;
    let y1 = update.coord.y;

    window
        .buffer
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, val)| {
            let x = index % width;
            let y = index / width;

            let c_r = x as f64 / update.zoom + x1;
            let c_i = y as f64 / update.zoom + y1;
            let mut z_r = 0.0;
            let mut z_i = 0.0;
            let mut i = 0;

            while (z_r * z_r + z_i * z_i < 4.0) && i < update.iter {
                let tmp = z_r;
                z_r = z_r * z_r - z_i * z_i + c_r;
                z_i = 2.0 * z_i * tmp + c_i;
                i += 1;
            }

            if i == update.iter {
                *val = std::u32::MAX;
            } else {
                *val = i as u32;
            }
        });
}
