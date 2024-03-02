FROM docker.io/blackdex/rust-musl:x86_64-musl as chef

RUN cargo install cargo-chef
WORKDIR /gedeair_backend

# --
FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --
FROM chef AS builder

COPY --from=planner /gedeair_backend/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

# --
FROM alpine:latest as certs

RUN apk --update add ca-certificates

# --
FROM scratch

ENV PATH=/bin
COPY --from=certs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /gedeair_backend/target/x86_64-unknown-linux-musl/release/gedeair_backend /gedeair_backend
ENTRYPOINT ["/gedeair_backend"]
EXPOSE 3000
