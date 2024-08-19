## Rust Web Application Starter

### Prerequisites

#### Rust

Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### WebAssembly

Install WebAssembly target

```bash
rustup target add wasm32-unknown-unknown
```

#### Install Trunk

Trunk can build, bundle & ship your Rust WASM application to the web.

```bash
cargo install --locked trunk
```

### Run

```bash
trunk serve
```