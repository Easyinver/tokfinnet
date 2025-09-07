#!/usr/bin/env bash
set -e

# Uso: ./launch-validators.sh <BOOTNODE_IP> <BOOTNODE_PEERID>

if [ "$#" -ne 2 ]; then
  echo "❌ Uso: $0 <BOOTNODE_IP> <BOOTNODE_PEERID>"
  exit 1
fi

BOOTNODE_IP=$1
BOOTNODE_PEERID=$2

echo "🔗 Configurando validadores con bootnode: /ip4/$BOOTNODE_IP/tcp/30333/p2p/$BOOTNODE_PEERID"

# Generar docker-compose.override.yml para inyectar el bootnode en validadores
cat > docker-compose.override.yml <<EOL
version: "3.9"

services:
  validator1:
    command: >
      tokfin-node
      --chain /etc/tokfinnet/tokfin_raw.json
      --base-path /data/validator1
      --name Validator1
      --validator
      --port 30334
      --ws-port 9945
      --rpc-port 9934
      --bootnodes /ip4/$BOOTNODE_IP/tcp/30333/p2p/$BOOTNODE_PEERID

  validator2:
    command: >
      tokfin-node
      --chain /etc/tokfinnet/tokfin_raw.json
      --base-path /data/validator2
      --name Validator2
      --validator
      --port 30335
      --ws-port 9946
      --rpc-port 9935
      --bootnodes /ip4/$BOOTNODE_IP/tcp/30333/p2p/$BOOTNODE_PEERID

  validator3:
    command: >
      tokfin-node
      --chain /etc/tokfinnet/tokfin_raw.json
      --base-path /data/validator3
      --name Validator3
      --validator
      --port 30336
      --ws-port 9947
      --rpc-port 9936
      --bootnodes /ip4/$BOOTNODE_IP/tcp/30333/p2p/$BOOTNODE_PEERID
EOL

echo "✅ docker-compose.override.yml generado"

# Lanzar validadores
docker compose up -d validator1 validator2 validator3
