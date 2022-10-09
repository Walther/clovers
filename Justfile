# List the available recipes
default:
  @just --list --unsorted

# Run the CLI application with the given parameters
cli *ARGS:
  cargo run --bin clovers-cli --release -- {{ARGS}}

# Build and open the documentation
docs:
  cargo doc --no-deps --open

# Render a basic test scene with default settings with the CLI
scene:
  cargo run --bin clovers-cli --release -- --input scenes/scene.json -w 1920 -h 1080

# Render all the test scenes available in the repository
all-scenes:
  DATE=$(date -u +%s); \
  mkdir -p renders/$DATE; \
  for scene in $(ls scenes/ |grep json); \
  do just cli -s 1 --input scenes/$scene --output renders/$DATE/${scene%.json}.png; \
  done;

# Temporary helper for experimental gltf rendering
gltf scene *ARGS:
  just cli --input ~/git/glTF-Sample-Models/2.0/{{scene}}/glTF/{{scene}}.gltf --output renders/{{scene}}.png {{ARGS}}

# Temporary helper for experimental gltf rendering
gltf-list:
  ls ~/git/glTF-Sample-Models/2.0/