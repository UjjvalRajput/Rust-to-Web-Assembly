# Rust WebAssembly Greeting Project

This project demonstrates creating a WebAssembly (Wasm) module using Rust, and interacting with it from a web page. The module includes a basic `greet` function that returns a greeting message.

## Project Structure

- `Cargo.toml`: Rust project configuration.
- `src/lib.rs`: Rust source code for the WebAssembly module.
- `index.html`: HTML file to load and interact with the WebAssembly module.
- `main.js`: JavaScript file to interact with the WebAssembly module.
- `pkg/`: Directory containing the compiled WebAssembly package. **NOTE**: Ideally, the build process (`wasm-pack build`) generates the `pkg` folder and its contents. However, this repository includes the `pkg` folder to facilitate demonstration and usage without requiring the user to perform the build step. If you want to rebuild the WebAssembly package from source, you can delete the `pkg` folder and run `wasm-pack build` to regenerate it.
## Getting Started

### 1. Install Dependencies

Ensure you have the following tools installed:

- Rust and Cargo: Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- `wasm-pack`: Install via `cargo install wasm-pack`.

### 2. Build the WebAssembly Module

Navigate to your project directory and run:

```sh
wasm-pack build --target web
http-server .
