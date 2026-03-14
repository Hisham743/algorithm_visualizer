# Algorithm Visualizer

A sorting algorithm visualizer built in Rust with [egui](https://github.com/emilk/egui)/[eframe](https://github.com/emilk/egui/tree/main/crates/eframe), targeting both desktop and WebAssembly.

**Live demo:** <https://hisham743.github.io/algorithm_visualizer>

## Algorithms

- Bubble Sort
- Selection Sort
- Insertion Sort
- Merge Sort
- Quick Sort
- Heap Sort
- Gnome Sort
- Cocktail Sort
- Odd Even Sort
- Radix Sort
- Shell Sort

## Running Natively

```sh
cargo run --release
```

## Running in the Browser (WASM)

Requires [Trunk](https://trunkrs.dev) and the WASM target:

```sh
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
trunk serve --release
```
