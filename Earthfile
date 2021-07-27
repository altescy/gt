FROM ekidd/rust-musl-builder:1.51.0
WORKDIR /work

build:
    COPY . ./
    RUN cargo build --release
    SAVE ARTIFACT target /target AS LOCAL target

test:
    COPY . ./
    RUN cargo test

docker:
    FROM alpine
    COPY +build/target/x86_64-unknown-linux-musl/release/gt /app/
    ENTRYPOINT ["/app/gt"]
    SAVE IMAGE gt
