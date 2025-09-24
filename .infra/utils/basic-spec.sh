#!/usr/bin/env bash

set -euo pipefail

SPEC_FILE=${1:-tokfinSpec.json}
RAW_FILE=${2:-tokfinRaw.json}

echo "📦 basic-spec.sh: SPEC_FILE=$SPEC_FILE RAW_FILE=$RAW_FILE"

# 1. Generar spec base
echo "🔧 Building base spec from node (dev) -> $SPEC_FILE"
./target/release/tokfin-node build-spec --chain=dev --disable-default-bootnode > "$SPEC_FILE"

# 2. Función para extraer claves
gen_keys () {
  NAME=$1
  echo "🔑 Generando claves para $NAME..." >&2

  SR25519=$(subkey inspect --scheme sr25519 //$NAME | grep "Public key (hex)" | awk '{print $4}')
  ED25519=$(subkey inspect --scheme ed25519 //$NAME | grep "Public key (hex)" | awk '{print $4}')

  echo "  account: 0x$SR25519" >&2
  echo "  babe:    0x$SR25519" >&2
  echo "  grandpa: 0x$ED25519" >&2

  jq -n \
    --arg acc "0x$SR25519" \
    --arg babe "0x$SR25519" \
    --arg grandpa "0x$ED25519" \
    '[ $acc, $acc, { babe: $babe, grandpa: $grandpa } ]'
}

# 3. Generar claves para todos
SESSION_KEYS_JSON=$(jq -s '.' \
  <(gen_keys Alice) \
  <(gen_keys Bob) \
  <(gen_keys Charlie) \
  <(gen_keys Dave) \
  <(gen_keys Eve)
)

# 4. Inyectar en patch.session.keys
echo "🔍 Validando SESSION_KEYS_JSON..."
echo "$SESSION_KEYS_JSON" | jq . > /dev/null

TMP_FILE=$(mktemp)
jq --argjson keys "$SESSION_KEYS_JSON" '.patch.session.keys = $keys' "$SPEC_FILE" > "$TMP_FILE"
mv "$TMP_FILE" "$SPEC_FILE"

echo "✅ Spec actualizado: $SPEC_FILE"

# 5. Generar raw spec
echo "⛏️  Generando raw spec -> $RAW_FILE"
./target/release/tokfin-node build-spec --chain="$SPEC_FILE" --raw --disable-default-bootnode > "$RAW_FILE"

echo "🎉 Done!"
