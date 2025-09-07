# ===============================
# Stage 1: Build
# ===============================
FROM rust:1.72 AS builder

WORKDIR /work

# Cache dependencies first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo fetch

# Copy full source
COPY . .
RUN cargo build --release -p tokfin-node

# ===============================
# Stage 2: Runtime image
# ===============================
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy node binary from builder
COPY --from=builder /work/target/release/tokfin-node /usr/local/bin/tokfin-node

# Copy chain spec (ya que confirmaste que existe en ./chain-spec)
COPY ./chain-spec/tokfin_raw.json /etc/tokfinnet/tokfin_raw.json

EXPOSE 30333 9944 9933

ENTRYPOINT ["/usr/local/bin/tokfin-node"]
CMD ["--base-path", "/data", "--chain", "/etc/tokfinnet/tokfin_raw.json"]
