## Rust Web Application Starter

### Prerequisites

#### Rust

Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Node.js And PNPM

Install Node.js:

```bash
# installs nvm (Node Version Manager)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
# download and install Node.js (you may need to restart the terminal)
nvm install 18
```

Install PNPM:

```bash
npm install -g pnpm
```

### Run

Before running the application, you need to install the dependencies:

```bash
pnpm run bootstrap
```

Then you can run the application:

```bash
pnpm run dev
```

### Build

To build the application:

```bash
pnpm run build
```

## License

[MIT @yugasun](LICENSE)