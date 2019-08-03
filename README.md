# Mandelbrust

![](base_mandel.png)
![](mandel1.png)
![](mandel2.png)
![](mandel3.png)
![](mandel4.png)
![](mandel5.png)
![](mandel_unzoomed.png)

This project draw a mandelbrot fractale.

It use the `minifb` crate for the window / drawing.

To build the project use:
```
cargo build --release
```

To run the project:
```
cargo run --release
```

## Control

- Use the arrow key to move on the fractal.
- Use `zqsd` to move
- Use `wasd` to move
- `i`: Augment the level of iteration
- `u`: Reduce the level of iteration
- `space`: Zoom in the fractal
- `x`: Unzoom the fractal
- `escape`: Exit
