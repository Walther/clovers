# clovers 🍀 ray tracing in rust 🦀

![Raytraced render of a Cornell box with a large ceiling light, one green wall, one red wall, one tall box, and a glass sphere.](scene.png)

## Usage

Currently, this project is highly experimental. Things change all the time.

- `clovers/` contains the library
- `clovers-cli/` contains the CLI application
- `clovers-app/` contains the GUI application
- `clovers-gpu/` contains the shaders for GPU accelerated rendering

The automatically built documentation is hosted at <https://walther.github.io/clovers/clovers/>.

This project has a [Justfile](https://github.com/casey/just). If you have `just` installed, you can run `just` to list the available commands in this project, and run various forms of `just command --option args` to execute them.

If you do not have `just` installed, open the [Justfile](./Justfile) in your favorite text editor to see the various available commands that you can then copy to your shell manually.

## License

This project does not currently have an open license, you can consider it proprietary software that happens to have its source visible.
I might pick an open license at some point, or make this project private.
No guarantees, sorry!

If you make a PR to this repository, please acknowledge that you are giving all rights to this project.

### Model files

This repository has some example model files for demonstrating triangle-based object imports in addition to the declarative object primitives.

- Utah Teapot model `teapot.stl` CC0 1.0 Universal Public Domain [Wikipedia](https://en.wikipedia.org/wiki/File:Utah_teapot_(solid).stl)
- Stanford Bunny model `bunny.stl` CC Attribution 3.0 Unported [Wikipedia](https://commons.wikimedia.org/wiki/File:Stanford_Bunny.stl)
- Stanford Dragon model `dragon.stl` (stl converted version) CC Attribution [Thingiverse](https://www.thingiverse.com/thing:27666)
