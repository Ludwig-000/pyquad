# Pyquad

This is a Python abstraction for the Rust engine [macroquad](https://github.com/not-fl3/macroquad).

The project is in early development and the repository exists for documentation purposes.


Featuring 2D:  
![2D Screenshot](./docs/2d_screenshot.png)

Featuring 3D:  
![3D Screenshot](./docs/3d_screenshot.png)


How to build:

1) Create and activate a python virtual enviroment.

2) Run `cargo run --bin stub_gen` to update `pyquad.pyi` and gennerate an up-to-date python stub.

3) Run `maturin build --release`

4) The .whl file can be found at `./target/wheels/`

5) The .whl file can be added to pip via `pip install C:\path\to\your\file.whl --force-reinstall`