use crate::window;

pub fn compute(window: &mut window::Window, max_iter: usize, zoom: f64) {
    let width = window.width();
    let height = window.height();

    let x1 = -0.1011;
    let y1 = 0.9563;

    for y in 0..height {
        for x in 0..width {
            let c_r = x as f64 / zoom + x1;
            let c_i = y as f64 / zoom + y1;
            let mut z_r = 0.0;
            let mut z_i = 0.0;
            let mut i = 0;

            while (z_r * z_r + z_i * z_i < 4.0) && i < max_iter {
                let tmp = z_r;
                z_r = z_r * z_r - z_i * z_i + c_r;
                z_i = 2.0 * z_i * tmp + c_i;
                i += 1;
            }

            if i == max_iter {
                window.buffer[x + y * width] = 0x00FFFFFF;
            } else {
                let rapport = std::u32::MAX / max_iter as u32;
                let i = i as u32 * rapport;
                window.buffer[x + y * width] = i;
            }
        }
    }
}
