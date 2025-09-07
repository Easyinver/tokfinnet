#!/usr/bin/env bash
set -e

# Uso: ./launch-bootnode.sh

echo "🚀 Lanzando Bootnode Tokfin..."

# Generar docker-compose.override.yml para bootnode
cat > docker-compose.override.yml <<EOL
version: "3.9"

services:
  bootnode:
    command: >
      tokfin-node
      --chain /etc/tokfinnet/tokfin_raw.json
      --base-path /data/bootnode
      --name BootnodeTokfin
      --node-key 0000000000000000000000000000000000000000000000000000000000000001
      --port 30333
      --ws-port 9944
      --rpc-port 9933
EOL

echo "✅ docker-compose.override.yml generado para bootnode"

# Lanzar bootnode
docker compose up -d bootnode

echo "ℹ️ Revisa logs con: docker compose logs -f bootnode"
echo "⚠️ Apunta el PeerId que aparezca en logs y compártelo con los validadores"
