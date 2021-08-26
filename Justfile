# List the available recipes
default:
  @just --list --unsorted

# Run a basic scene with default settings with the CLI
scene:
  cargo run --bin clovers --release -- --input clovers/scenes/scene.json

# Run the CLI application with the given parameters
cli *ARGS:
  cargo run --bin clovers --release -- {{ARGS}}
