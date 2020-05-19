mod window;

use mandelbrust::{color, Mandel};
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
    window.update(); // if I don’t do it twice I get a gray screen

    while window.handle_event(&mut mandel) {
        let (width, height) = window.dimension();
        mandel.compute(&mut window.buffer, width, height);
        color::convert_nb_to_rbg(mandel.iter, &mut window.buffer);

        window.update();
    }
}
