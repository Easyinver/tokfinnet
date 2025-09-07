
# 🛠️ Montar un nodo Tokfin desde cero (sin Docker)

Este tutorial explica cómo instalar todas las dependencias y compilar un nodo Tokfin en una máquina **Ubuntu 22.04** limpia.
Úsalo si quieres desplegar nodos manualmente, sin contenedores.

---

## 1. Actualizar sistema

```bash
sudo apt update && sudo apt upgrade -y
```

---

## 2. Instalar dependencias de compilación

```bash
sudo apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler libclang-dev
sudo apt install --assume-yes build-essential pkg-config libzstd-dev libjemalloc-dev autoconf libtool
```

---

## 3. Instalar Node.js (para herramientas relacionadas)

```bash
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs
```

---

## 4. Instalar Rust y toolchain de Substrate

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup default stable
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
rustup show
rustup +nightly show
```

---

## 5. Clonar Tokfin

```bash
git clone -b EspecializedNodes https://github.com/easyinver/tokfinnet.git ~/tokfinnet
cd ~/tokfinnet
```

---

## 6. Compilar el nodo

```bash
cargo build --release -p tokfin-node
```

---

## 7. Instalar el binario en el sistema

```bash
sudo cp target/release/tokfin-node /usr/local/bin/
```

---

## 8. Lanzar un nodo

Ejemplo para correr como **validador** con la chain spec inicial:

```bash
tokfin-node \
  --chain ~/tokfin/chain-specs/tokfin_raw.json \
  --validator \
  --name MyValidator
```

---

## 🔗 Script automatizado

Si prefieres no escribir todos los comandos, puedes usar el script incluido en el repo:

```bash
./.infra/utils/setup-node.sh
```

Este script hace todos los pasos anteriores automáticamente.

---

Así ya tienes un camino claro:

* Para producción → usas Docker/compose.
* Para debugging, auditoría o fallback → lo levantas manualmente con este tutorial.

---

