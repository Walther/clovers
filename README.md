# clovers 🍀 ray tracing in rust 🦀

![Raytraced render of a Cornell box with a large ceiling light, one green wall, one red wall, one tall box, and a glass sphere.](scene.png)

## Usage

Currently, this project is highly experimental. Things change all the time.

- `clovers/` contains the library
- `clovers-cli/` contains the CLI application
- `clovers-app/` contains the GUI application
- `clovers-gpu/` contains the shaders for GPU accelerated rendering

Run `cargo doc --open` to build and open the documentation.

## License

This project does not currently have an open license, you can consider it proprietary software that happens to have its source visible.
I might pick an open license at some point, or make this project private.
No guarantees, sorry!

If you make a PR to this repository, please acknowledge that you are giving all rights to this project.

### Model files

This repository has some example model files for demonstrating triangle-based object imports in addition to the declarative object primitives.

- Utah Teapot model `teapot.stl` CC0 1.0 Universal Public Domain [Wikipedia](https://en.wikipedia.org/wiki/File:Utah_teapot_(solid).stl)
- Stanford Bunny model `bunny.stl` CC Attribution 3.0 Unported [Wikipedia](https://commons.wikimedia.org/wiki/File:Stanford_Bunny.stl)
