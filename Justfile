# List the available recipes
default:
  @just --list --unsorted

# Run the CLI application with the given parameters
cli *ARGS:
  cargo run --bin clovers-cli --release -- {{ARGS}}

# Build and open the documentation
doc:
  cargo doc --no-deps --open

# Render all the test scenes available in the repository
all-scenes *ARGS:
  DATE=$(date -u +%s); \
  mkdir -p renders/$DATE; \
  for scene in $(ls scenes/ |grep json); \
  do just cli render --output renders/$DATE/${scene%.json} {{ARGS}} -i scenes/$scene; \
  done;

# Profiling helper
profile *ARGS:
  cargo build --profile profiling; \
  samply record -- ./target/profiling/clovers-cli {{ARGS}}

# Run all tests
test:
  cargo nextest run --cargo-quiet

# Run all benchmarks
bench *ARGS:
  cargo bench --quiet {{ARGS}}

# Verify no_std compatibility
nostd:
  cargo clippy -q --release --package clovers --lib --no-default-features

# Check the assembly of a given function
asm name:
  cargo asm --release --package clovers --lib --no-default-features {{name}}
