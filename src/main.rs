mod mandel;
mod window;

const HEIGHT: usize = 800;
const WIDTH: usize = 800;

fn main() {
    let mut mandel = mandel::Mandel::new(-0.1011, 0.9563, 50, 10_000.0);
    let mut window = window::Window::new(WIDTH, HEIGHT).unwrap();

    // init window
    mandel.compute(&mut window);
    window.update();

    while window.handle_event(&mut mandel) {
        mandel.compute(&mut window);

        window.update();
    }
}
