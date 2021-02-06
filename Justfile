default:
  @just --list

scene:
  pushd clovers; cargo run --bin clovers --release --features "cli" -- --input scenes/scene.json; popd;

gui:
  pushd clovers-app; cargo run --release -- --input ../clovers/scenes/scene.json; popd;