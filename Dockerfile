# Build stage for Rust/WASM
FROM rust:1.86-slim AS wasm-builder

# Install wasm-pack and build dependencies
RUN apt-get update && \
    apt-get install -y curl && \
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

WORKDIR /app
COPY src/rust ./src/rust
COPY Cargo.toml ./
COPY Cargo.lock ./

# Build WASM package
RUN wasm-pack build --target web --release --out-dir wasm

# Bun build stage
FROM node:23-slim AS builder

WORKDIR /app

COPY package.json ./

RUN npm install

# Copy WASM build output
COPY --from=wasm-builder /app/wasm ./wasm
COPY . .

RUN node setup/cache_comp.js WC2025

RUN npm run build

# Final nginx deploy stage
FROM nginx:1.27-alpine-slim

COPY --from=builder /app/dist /usr/share/nginx/html

RUN rm /etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]