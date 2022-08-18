VERSION 0.6
FROM rust:alpine
WORKDIR /neo

# Install packages and CLI dependencies here
install:
  RUN apk update
  RUN apk add pkgconfig build-base zlib openssl-dev mingw-w64-gcc

# Build phases

build-windows:
  FROM +install

  RUN rustup target add x86_64-pc-windows-gnu
  COPY --dir src manifest.xml icon.ico build.rs Cargo.lock Cargo.toml ./
  RUN cargo build --target x86_64-pc-windows-gnu --release 
  SAVE ARTIFACT ./target/x86_64-pc-windows-gnu/release/neo.exe neo.exe AS LOCAL ./release/

build-linux:
  FROM +install

  RUN rustup target add x86_64-unknown-linux-musl
  COPY --dir src build.rs Cargo.lock Cargo.toml ./
  RUN cargo build --release
  SAVE ARTIFACT ./target/release/neo neo AS LOCAL ./release/