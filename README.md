# clovers 🍀

Physically based spectral path tracing renderer with a focus on high-quality offline rendering

![a giant floating glass prism, a small glass sphere, and lights interacting in a virtual scene. the white light gets split by the prism and throws a spectral rainbow on the gray floor](dispersive.jpg)
![the stanford dragon model rendered in iridescent colors, a subtle blend of orange with a hint of magenta](dragon.jpg)

## Usage

Currently, this project is highly experimental. Things change all the time.

- `clovers/` contains the core library and types
- `clovers-cli/` contains the CLI application & runtime

The automatically built documentation is hosted at <https://walther.github.io/clovers/clovers/>.

This project has a [Justfile](https://github.com/casey/just). If you have `just` installed, you can run `just` to list the available commands in this project, and run various forms of `just command --option args` to execute them.

If you do not have `just` installed, open the [Justfile](./Justfile) in your favorite text editor to see the various available commands that you can then copy to your shell manually.

## License

This project does not currently have an open license, you can consider it proprietary software that happens to have its source visible.
I might pick an open license at some point, or make this project private.
No guarantees, sorry!

If you make a PR to this repository, please acknowledge that you are giving all rights to this project.

### Model files

This repository has some example model files for demonstrating triangle-based object imports in addition to the declarative object primitives. Check the following directories:

- `stl/`
- `ply/`
- `gltf/`

## Useful references

Making this renderer would not have been possible without the availability of an abundance of great literature. Here are listed some of the sources that have been useful:

- [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
- [Ray Tracing: The Next Week](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
- [Ray Tracing: The Rest of Your Life](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)
- [Physically Meaningful Rendering using Tristimulus Colours](https://doi.org/10.1111/cgf.12676)
- [Hero Wavelength Spectral Sampling](https://doi.org/10.1111/cgf.12419)
- [How to interpret the sRGB color space](https://color.org/chardata/rgb/sRGB.pdf)
- [Physically Based Rendering: From Theory To Implementation. 4ed](https://pbr-book.org/)
