#!/usr/bin/env bash
set -e

echo "🚀 Tokfin full node setup (Ubuntu 22.04 clean machine)"

# 1. Actualizar sistema
sudo apt update && sudo apt upgrade -y

# 2. Dependencias generales
sudo apt install -y build-essential git clang curl \
  pkg-config libssl-dev libclang-dev cmake

# 3. Node.js (para herramientas relacionadas, UI si hace falta)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs

# 4. Rust toolchain + Substrate deps
url --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh - -- -y
source $HOME/.cargo/env
rustup default stable
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
rustup show
rustup +nightly show

# 5. Clonar Tokfin repo
if [ ! -d "$HOME/tokfinnet" ]; then
  git clone -b EspecializedNodes https://github.com/easyinver/tokfinnet.git $HOME/tokfinnet
fi

cd $HOME/tokfinnet

# 6. Compilar nodo
cargo build --release -p tokfin-node

# 7. (Opcional) copiar binario a /usr/local/bin
sudo cp target/release/tokfin-node /usr/local/bin/

echo "✅ Tokfin node ready. Run with:"
echo "   tokfin-node --chain ./chain-specs/tokfin_raw.json --validator ..."
