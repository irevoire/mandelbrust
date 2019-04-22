use std::time::Instant;

mod mandel;
mod window;

const HEIGHT: usize = 800;
const WIDTH: usize = 800;

fn main() {
    let mut window = window::Window::new(WIDTH, HEIGHT).unwrap();
    // init window
    window.update();

    while window.is_open() {
        let now = Instant::now();
        mandel::compute(&mut window, 50, 10_000.0);
        let duration = now.elapsed();
        println!(
            "{:.5} seconds for whatever you did.",
            duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
        );
        window.update();
    }
}
