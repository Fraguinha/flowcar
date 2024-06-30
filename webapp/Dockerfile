FROM rust:alpine AS builder

RUN apk update && apk add --no-cache bash npm libc-dev binaryen

RUN npm install -g sass

RUN cargo install cargo-leptos

WORKDIR /build
COPY . .

RUN rustup target add wasm32-unknown-unknown

RUN cargo leptos build --release -vv

FROM rust:alpine AS runner

WORKDIR /app

COPY --from=builder /build/target/release/flowcar /app/
COPY --from=builder /build/target/site /app/site
COPY --from=builder /build/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"

EXPOSE 8080

CMD ["/app/flowcar"]
