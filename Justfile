# List the available recipes
default:
  @just --list --unsorted

# Run a basic scene with default settings with the CLI
scene:
  pushd clovers; cargo run --bin clovers --release --features "cli" -- --input scenes/scene.json; popd;

# Run a basic scene with default settings with the GUI
gui:
  pushd clovers-app; cargo run --release -- --input ../clovers/scenes/scene.json; popd;

# Run the CLI application with the given parameters
cli *ARGS:
  pushd clovers; cargo run --bin clovers --release --features "cli" -- {{ARGS}}