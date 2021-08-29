# List the available recipes
default:
  @just --list --unsorted

# Run a basic scene with default settings with the CLI
scene:
  cargo run --bin clovers --release -- --input scenes/scene.json

# Run the CLI application with the given parameters
cli *ARGS:
  cargo run --bin clovers --release -- {{ARGS}}

# Render all the test scenes available in the repository
all-scenes:
  DATE=$(date -Is); \
  mkdir -p renders/$DATE; \
  for scene in $(ls scenes/ |grep json); \
  do just cli --input scenes/$scene --output renders/$DATE/${scene%.json}.png; \
  done;
