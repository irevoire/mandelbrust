mod mandel;
use fractal::*;

const HEIGHT: usize = 800;
const WIDTH: usize = 800;

fn main() {
    let mut mandel = Update {
        coord: Coord { x: -2.3, y: -1.4 },
        iter: 10,
        zoom: 250.0,
    };
    let mut window = Window::new("Mandelbrüst", WIDTH, HEIGHT).unwrap();

    // init window
    mandel::compute(&mut mandel, &mut window);
    window.draw(&mandel);

    while window.handle_event(&mut mandel) {
        mandel::compute(&mandel, &mut window);

        window.draw(&mandel);
    }
}
