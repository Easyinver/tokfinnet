# ===== BUILDER =====
FROM rust:1.72 as builder

WORKDIR /work

# Configurar toolchains de Rust igual que en tu máquina
RUN rustup default stable && \
    rustup update && \
    rustup update nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly && \
    rustup component add rust-src --toolchain nightly && \
    rustup show && \
    rustup +nightly show

# Instalar dependencias del sistema
RUN apt-get update && apt-get install -y \
    git clang curl libssl-dev llvm libudev-dev make protobuf-compiler \
    build-essential pkg-config libzstd-dev libjemalloc-dev autoconf libtool \
    && rm -rf /var/lib/apt/lists/*

# Copiar el repo completo
COPY . .

# Compilar con nightly (para runtime Substrate)
RUN cargo +nightly build --release -p tokfin-node
# Compilamos el nodo en release
RUN cargo build --release -p tokfin-node

# ===== RUNTIME IMAGE =====
FROM debian:bookworm-slim as runtime

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# copia el binario compilado desde el builder
COPY --from=builder /work/target/release/tokfin-node /usr/local/bin/tokfin-node

# copia el chain spec generado en la stage anterior
COPY --from=builder /work/chain-spec/tokfin_raw.json /etc/tokfinnet/tokfin_raw.json


EXPOSE 30333 9944 9933

ENTRYPOINT ["/usr/local/bin/tokfin-node"]
