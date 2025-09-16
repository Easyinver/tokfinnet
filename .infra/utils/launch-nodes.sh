#!/bin/bash
# ==========================
# Tokfin Node Launcher Script
# ==========================

# Cómo usarlo
#nano tokfin-node-start.sh
#chmod +x tokfin-node-start.sh

#Edita las variables según el nodo:
# En el bootnode, rellena NODE_KEY_HEX y deja BOOTNODE_MULTIADDR vacío.
# En los validadores, deja NODE_KEY_HEX="" y pon la dirección multiaddr del bootnode en BOOTNODE_MULTIADDR.
# ./tokfin-node-start.sh

# === CONFIGURACIÓN ===
NODE_NAME="Bootnode"               # Nombre del nodo ("Bootnode", "Alice", "Bob"...)
BASE_PATH="/var/lib/tokfin/bootnode"   # Directorio persistente
CHAIN_SPEC="/root/tokfinnet/tokfinSpecRaw.json"
PORT=30333
RPC_PORT=9944
WS_PORT=9945

IS_BOOTNODE=true   # true = bootnode, false = validador
BOOTNODE_MULTIADDR="/ip4/<IP_BOOTNODE>/tcp/30333/p2p/<PEER_ID>"  # Solo para validadores


# === PREPARAR DIRECTORIO ===
mkdir -p "$BASE_PATH/network"

NODE_KEY_FILE="$BASE_PATH/network/secret_ed25519"

# === BOOTNODE: generar clave si no existe ===
if [ "$IS_BOOTNODE" = true ]; then
  if [ ! -f "$NODE_KEY_FILE" ]; then
    echo ">> Generando node-key para el bootnode..."
    ./target/release/tokfin-node key generate-node-key --file "$NODE_KEY_FILE"
  fi

  PEER_ID=$(./target/release/tokfin-node key inspect-node-key --file "$NODE_KEY_FILE")
  echo "====================================================="
  echo " Bootnode Peer ID: $PEER_ID"
  echo " Dirección Multiaddr:"
  echo "   /ip4/<IP_BOOTNODE>/tcp/$PORT/p2p/$PEER_ID"
  echo "====================================================="
fi


# === COMANDO DE ARRANQUE ===
CMD="./target/release/tokfin-node \
  --base-path $BASE_PATH \
  --chain $CHAIN_SPEC \
  --name $NODE_NAME \
  --port $PORT \
  --rpc-port $RPC_PORT \
  --ws-port $WS_PORT"

if [ "$IS_BOOTNODE" = true ]; then
  CMD="$CMD --node-key $(cat $NODE_KEY_FILE)"
else
  CMD="$CMD --validator --bootnodes $BOOTNODE_MULTIADDR"
fi

echo ">> Lanzando nodo: $NODE_NAME"
echo ">> Base path: $BASE_PATH"
exec $CMD
