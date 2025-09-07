#!/usr/bin/env bash
set -e

# 1. Levantar solo el bootnode
echo "🚀 Iniciando bootnode..."
docker compose up -d bootnode

# 2. Esperar un poco a que arranque
echo "⏳ Esperando a que arranque bootnode..."
sleep 5

# 3. Obtener el PeerId del bootnode
PEER_ID=$(docker logs bootnode 2>&1 | grep "Local node identity is" | awk '{print $NF}')
if [ -z "$PEER_ID" ]; then
  echo "❌ No se pudo obtener PeerId del bootnode"
  exit 1
fi

echo "✅ PeerId del bootnode: $PEER_ID"

# 4. Generar docker-compose con el PeerId incrustado
echo "🛠️ Generando docker-compose.generated.yml..."
cat > docker-compose.generated.yml <<EOF
version: "3.8"

services:
  bootnode:
    image: tokfin-node:latest
    container_name: bootnode
    command: >
      --chain=/etc/tokfinnet/tokfin_raw.json
      --base-path /data
      --node-key 0000000000000000000000000000000000000000000000000000000000000001
      --port 30333
      --ws-port 9944
      --rpc-port 9933
      --rpc-cors all
      --rpc-methods=Unsafe
      --name Bootnode
    volumes:
      - ./chain-spec:/etc/tokfinnet
      - ./data/bootnode:/data
    ports:
      - "30333:30333"
      - "9944:9944"
      - "9933:9933"

  validator1:
    image: tokfin-node:latest
    container_name: validator1
    command: >
      --chain=/etc/tokfinnet/tokfin_raw.json
      --base-path /data
      --port 30334
      --ws-port 9945
      --rpc-port 9934
      --rpc-cors all
      --rpc-methods=Unsafe
      --validator
      --name Alice
      --bootnodes /ip4/bootnode/tcp/30333/p2p/$PEER_ID
    depends_on:
      - bootnode
    volumes:
      - ./chain-spec:/etc/tokfinnet
      - ./data/validator1:/data
    ports:
      - "30334:30334"
      - "9945:9945"
      - "9934:9934"

  validator2:
    image: tokfin-node:latest
    container_name: validator2
    command: >
      --chain=/etc/tokfinnet/tokfin_raw.json
      --base-path /data
      --port 30335
      --ws-port 9946
      --rpc-port 9935
      --rpc-cors all
      --rpc-methods=Unsafe
      --validator
      --name Bob
      --bootnodes /ip4/bootnode/tcp/30333/p2p/$PEER_ID
    depends_on:
      - bootnode
    volumes:
      - ./chain-spec:/etc/tokfinnet
      - ./data/validator2:/data
    ports:
      - "30335:30335"
      - "9946:9946"
      - "9935:9935"

  validator3:
    image: tokfin-node:latest
    container_name: validator3
    command: >
      --chain=/etc/tokfinnet/tokfin_raw.json
      --base-path /data
      --port 30336
      --ws-port 9947
      --rpc-port 9936
      --rpc-cors all
      --rpc-methods=Unsafe
      --validator
      --name Charlie
      --bootnodes /ip4/bootnode/tcp/30333/p2p/$PEER_ID
    depends_on:
      - bootnode
    volumes:
      - ./chain-spec:/etc/tokfinnet
      - ./data/validator3:/data
    ports:
      - "30336:30336"
      - "9947:9947"
      - "9936:9936"
EOF

# 5. Levantar validadores
echo "🚀 Iniciando validadores con PeerId correcto..."
docker compose -f docker-compose.generated.yml up -d validator1 validator2 validator3

echo "🎉 Red Tokfin testnet levantada con bootnode + 3 validadores"
