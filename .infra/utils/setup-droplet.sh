#!/bin/bash
set -e

echo "==> [1/3] Configurando swap de 4GB..."
if [ ! -f /swapfile ]; then
  fallocate -l 4G /swapfile
  chmod 600 /swapfile
  mkswap /swapfile
  swapon /swapfile
  echo '/swapfile none swap sw 0 0' >> /etc/fstab
else
  echo "Swap ya existe, saltando..."
fi
swapon --show
free -h

echo "==> [2/3] Instalando dependencias básicas..."
apt-get update
apt-get install -y build-essential clang cmake pkg-config libssl-dev git curl

echo "==> [3/3] Instalando Rustup..."
if ! command -v cargo &> /dev/null; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source $HOME/.cargo/env
else
  echo "Rust ya instalado."
fi

echo "==> Todo listo ✅"
echo "Ahora entra en el proyecto y ejecuta:"
echo "  cd ~/tokfinnet"
echo "  cargo build --release -j 1"
