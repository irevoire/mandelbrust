mod color;
mod mandel;
mod window;

const HEIGHT: usize = 800;
const WIDTH: usize = 800;

fn main() {
    let mut mandel = mandel::Mandel::new(-2.3, -1.4, 10, 250.0);
    let mut window = window::Window::new(WIDTH, HEIGHT).unwrap();

    // init window
    let (width, height) = window.dimension();
    mandel.compute(&mut window.buffer, width, height);
    color::convert_nb_to_rbg(mandel.iter, &mut window.buffer);
    window.update();

    while window.handle_event(&mut mandel) {
        let (width, height) = window.dimension();
        mandel.compute(&mut window.buffer, width, height);
        color::convert_nb_to_rbg(mandel.iter, &mut window.buffer);

        window.update();
    }
}
