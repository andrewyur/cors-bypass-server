#!/bin/bash

docker run --rm -it -v "$(pwd)":/home/rust/src ghcr.io/rust-cross/rust-musl-cross:x86_64-musl sh -c "cargo build --release && musl-strip /home/rust/src/target/x86_64-unknown-linux-musl/release/cors-bypass-server"

docker run --rm -it -v "$(pwd)":/home/rust/src ghcr.io/rust-cross/rust-musl-cross:aarch64-musl sh -c "cargo build --release && musl-strip /home/rust/src/target/aarch64-unknown-linux-musl/release/cors-bypass-server"