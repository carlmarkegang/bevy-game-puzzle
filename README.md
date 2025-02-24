# bevy-game-puzzle 
Click here to play: https://carlmarkegang.itch.io/gravity-falling-matching-four-bevy-game

## Commands
 * Setup new folder: cd bevy_game_creature + cargo init + cargo add bevy
 * For exe: cargo run
 * For web: cargo run --target wasm32-unknown-unknown 
 * Remove dependancys and lock file: cargo clean
 * cargo build --release
 * cargo fix  - Auto clean project
 * Clear-History

## Web build for release
 * rustup target add wasm32-unknown-unknown
 * cargo install wasm-bindgen-cli
 * cargo build --target wasm32-unknown-unknown --profile wasm-release
 * Build will land in: target/wasm32-unknown-unknown/wasm-release/
 * wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "mygame" ./target/wasm32-unknown-unknown/wasm-release/bevy-game-puzzle.wasm
 * The final list of files for a minimal website will look something like this:: assets/ index.html mygame.js mygame_bg.wasm

## Auto build on update
cargo watch -x run

## Faster Compile times
cargo run --features bevy/dynamic_linking
