use minifb::{Scale, Window, WindowOptions};
use std::time::Instant;

const HEIGHT: usize = 800;
const WIDTH: usize = 800;
const MAX_ITER: usize = 100;

fn compute_mandel(width: usize, height: usize, buffer: &mut [u32]) {
    let x1 = -0.1011;
    let y1 = 0.9563;
    let zoom = 10_000.0;

    for y in 0..height {
        for x in 0..width {
            let c_r = x as f64 / zoom + x1;
            let c_i = y as f64 / zoom + y1;
            let mut z_r = 0.0;
            let mut z_i = 0.0;
            let mut i = 0;

            while (z_r * z_r + z_i * z_i < 4.0) && i < MAX_ITER {
                let tmp = z_r;
                z_r = z_r * z_r - z_i * z_i + c_r;
                z_i = 2.0 * z_i * tmp + c_i;
                i += 1;
            }

            if i == MAX_ITER {
                buffer[x + y * width] = 0x00FFFFFF;
            } else {
                let rapport = std::u32::MAX / MAX_ITER as u32;
                // let i = i as u8 * rapport;
                // buffer[x + y * width] = ((i << 3) | (i << 2) | i).into();
                let i = i as u32 * rapport;
                buffer[x + y * width] = i;
            }
        }
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = match Window::new(
        "Mandelbrot",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false, // TODO
            scale: Scale::X1,
            ..WindowOptions::default()
        },
    ) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };
    // init window
    window.update_with_buffer(&buffer).unwrap();

    let (width, height) = (WIDTH, HEIGHT);

    while window.is_open() {
        let now = Instant::now();
        compute_mandel(width, height, &mut buffer);
        let duration = now.elapsed();
        println!(
            "{:.5} seconds for whatever you did.",
            duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
        );
        // We unwrap here as we want this code to exit if it fails
        window.update_with_buffer(&buffer).unwrap();
    }
}
