# Tokfin Testnet (Infra)

Este directorio contiene la configuración necesaria para levantar la **testnet de Tokfin** usando Docker.

## 📦 Requisitos

- Docker y Docker Compose instalados
- Clonar este repositorio (`tokfinnet`)
- Compilar previamente la imagen:
  ```bash
  docker build -t tokfin-node:latest .


# Operación de la testnet Tokfin

## Verificar consistencia del runtime

Todos los nodos deben usar **el mismo runtime WASM** para evitar forks en la red.

Antes de lanzar nodos en la nube, comprueba el hash del runtime con:

```bash
./target/release/tokfin-node inspect-node --chain ./chain-specs/tokfin.json | grep -A3 "Runtime"

