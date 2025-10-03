# Pyquad

This is a Python abstraction for the Rust engine [macroquad](https://github.com/not-fl3/macroquad).

The project is in early development and the repository exists for documentation purposes.

#
## Featuring 2D:  
![2D Screenshot](https://raw.githubusercontent.com/Ludwig-000/pyquad/refs/heads/main/docs/2d_screenshot.png)
#
## and 3D:
![3D Screenshot](https://raw.githubusercontent.com/Ludwig-000/pyquad/refs/heads/main/docs/3d_screenshot.png)

#
#
**Hello rectangle:**
```python
from pyquad import *

activate_engine()

while True:
    draw_rectangle(100, 100, 600, 300, Color.GREEN)
    draw_text("Hello rectangle", 200, 200, 50, Color.RED)
    next_frame()
```
#
How to build:

1) Create and activate a python virtual environment.

2) Run `cargo run --bin stub_gen` to update `pyquad.pyi` and gennerate an up-to-date python stub.

3) Run `maturin build --release`

4) The .whl file can be found at `./target/wheels/`

5) The .whl file can be added to pip via `pip install C:\path\to\your\file.whl --force-reinstall`