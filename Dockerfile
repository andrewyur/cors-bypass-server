ARG IMAGE_TAG=x86_64-musl
FROM ghcr.io/rust-cross/rust-musl-cross:${IMAGE_TAG} AS builder
WORKDIR /app
COPY . .

ARG TARGETARCH

RUN rustup target add aarch64-unknown-linux-musl x86_64-unknown-linux-musl

RUN cargo build --release \
    --target=$( if [ "$TARGETARCH" = "arm64" ]; then echo "aarch64-unknown-linux-musl"; else echo "x86_64-unknown-linux-musl"; fi)

WORKDIR /target
RUN mv /app/target/$( if [ "$TARGETARCH" = "arm64" ]; then echo "aarch64-unknown-linux-musl"; else echo "x86_64-unknown-linux-musl"; fi)/release/cors-bypass-server .

RUN /usr/local/musl/bin/musl-strip ./cors-bypass-server

# Stage 2: minimal distroless
FROM gcr.io/distroless/static-debian12:nonroot

ARG TARGETARCH
COPY --from=builder /target /app/
USER nonroot:nonroot

ENTRYPOINT ["/app/cors-bypass-server"]