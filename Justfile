# List the available recipes
default:
  @just --list --unsorted

# Run a basic scene with default settings with the CLI
scene:
  cargo run --manifest-path clovers/Cargo.toml --bin clovers --release -- --input clovers/scenes/scene.json; popd;

# Run a basic scene with default settings with the GUI
gui:
  cargo run --manifest-path clovers-app/Cargo.toml --release -- --input clovers/scenes/scene.json;

# Run the CLI application with the given parameters
cli *ARGS:
  cargo run --manifest-path clovers/Cargo.toml --bin clovers --release -- {{ARGS}}

