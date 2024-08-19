# Rust WebAssembly Greeting Project

This project demonstrates creating a WebAssembly (Wasm) module using Rust, and interacting with it from a web page. The module includes a basic `greet` function that returns a greeting message.

## Project Structure

- `Cargo.toml`: Rust project configuration.
- `src/lib.rs`: Rust source code for the WebAssembly module.
- `index.html`: HTML file to load and interact with the WebAssembly module.
- `main.js`: JavaScript file to interact with the WebAssembly module.
- `pkg/`: Directory containing the compiled WebAssembly package.

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
