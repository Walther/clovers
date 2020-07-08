# clovers 🍀 ray tracing in rust 🦀

## Usage

Currently, this project is highly experimental. Things change all the time.

- Clone the repository locally
- Run `cargo build --features "cli"` to build the command line interface only
- Run `cargo build --features "cli, gui"` to build the cli with optional gui window showing the render
- Modify the code in order to change the scene that gets rendered. See [issue 6](https://github.com/Walther/clovers/issues/6)
- Find your executable in `./target/release/`
- Run `clovers --help` to see the usage instructions for the binary
