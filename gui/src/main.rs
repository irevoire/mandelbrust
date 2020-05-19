mod window;

use mandelbrust::{color, Mandel};
use std::time::Instant;
use window::Window;

const HEIGHT: usize = 800;
const WIDTH: usize = 800;

fn main() {
    let mut mandel = Mandel::new(-1.3, -0.4, 15, 800.0);
    let mut window = Window::new(WIDTH, HEIGHT).unwrap();

    // init window
    let (width, height) = window.dimension();
    mandel.compute(&mut window.buffer, width, height);
    color::convert_nb_to_rbg(mandel.iter, &mut window.buffer);
    window.update();

    while window.handle_event(&mut mandel) {
        let now = Instant::now();

        let (width, height) = window.dimension();
        mandel.compute(&mut window.buffer, width, height);
        color::convert_nb_to_rbg(mandel.iter, &mut window.buffer);

        println!(
            "mandelbrot {:4} for {} iter",
            now.elapsed().as_secs_f32(),
            mandel.iter
        );
        let now = Instant::now();

        window.update();

        println!("refresh {:?}", now.elapsed().as_secs_f32());
    }
}
