# wca-odds-web

A WebAssembly port of the WCA odds calculator.

You can visit the deployed website here: **[odds.nmckee.org](https://odds.nmckee.org/)**

---

## Running Locally

This guide will help you set up the project on your local machine for development and testing.

### Prerequisites

First, ensure you have the following tools installed:

- [Rust](https://www.rust-lang.org/tools/install) and `cargo`
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
- [`bun`](https://bun.sh/docs/installation)

### Development Server

To run the local development server, follow these steps:

```bash
# 1. Compile the Rust code to WebAssembly
#    The output is placed in the 'wasm' directory
wasm-pack build --target web --out-dir wasm

# 2. Install project dependencies
bun install

# 3. Pre-cache common competitions
bun run setup/cache_comp.js setup/cache_list.json

# 4. Start the development server
bun dev
```

### Production Build

You can create and run a production-ready build using either Docker (recommended) or by building manually.

#### Using Docker (Recommended)

The simplest way to run a production build is with Docker. From the project root, run:

```bash
docker compose up
```

#### Manual Build

If you prefer to build the project without Docker:

```bash
# 1. Compile Rust code with release optimizations
wasm-pack build --target web --release --out-dir wasm

# 2. Install project dependencies
bun install

# 3. Pre-cache common competitions
bun run setup/cache_comp.js setup/cache_list.json

# 4. Create the production build
bun run build

# 5. Preview the final build
bun preview
```

\*Note: This project uses bun for local development. The Docker configuration uses NodeJS due to some build compatibility issues in the Docker environment.
