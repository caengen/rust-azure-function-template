# Azure functions in Rust

Template for a simple Azure Rust Function. Tested with Rust 1.6.

## Pre-requisites

1. Install [Rustup](https://rustup.rs/) and use it to install Rust.
2. Install [Azure Functions Core Tools](https://github.com/Azure/azure-functions-core-tools).
3. If using VS Code install the extensions `rust-analyser` and `Azure Functions`.

## Development

### Build

In the root of the project you can:

1. Run the function directly with `cargo run` (will start the `axum` server, note the port number).

or

2. Build the project with `cargo build --release` and start the server through the Azure Functions Core Tools, `func start --verbose`.
