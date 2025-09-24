#!/bin/bash
set -e

SPEC_FILE=${1:-tokfinSpec.json}
RAW_FILE=${2:-tokfinRaw.json}

echo "📦 basic-spec.sh: SPEC_FILE=$SPEC_FILE RAW_FILE=$RAW_FILE"

# Función para limpiar JSON
clean_json() {
    local file=$1
    echo "🧹 Limpiando notación científica en $file"
    
    sed -i 's/1e+21/"1000000000000000000000"/g' "$file"
    sed -i 's/1e+18/"1000000000000000000000"/g' "$file"
    sed -i 's/1e+12/"1000000000000"/g' "$file"
    sed -i 's/1e+9/"1000000000"/g' "$file"
    
    # Limpiar caracteres problemáticos
    sed -i 's/[[:cntrl:]]*$//' "$file"
    sed -i 's/[ \t]*$//' "$file"
}

# 1. Generar chainspec base
echo "🔧 Building base spec from node (dev) -> $SPEC_FILE"
./target/release/tokfin-node build-spec --disable-default-bootnode --chain dev > "$SPEC_FILE"

# Limpiar notación científica del spec base
clean_json "$SPEC_FILE"

# 2. Generar claves conocidas para los validadores
echo "🔑 Generando claves para validadores conocidos..."
VALIDATORS=("Alice" "Bob" "Charlie" "Dave" "Eve")

# Arrays para almacenar las claves
declare -a ACCOUNTS
declare -a BABE_KEYS 
declare -a GRANDPA_KEYS

for i in "${!VALIDATORS[@]}"; do
    validator="${VALIDATORS[$i]}"
    echo "🔑 Generando claves para $validator..."
    
    # Generar claves SR25519 (BABE)
    babe_output=$(./target/release/tokfin-node key inspect "//$validator" --scheme sr25519 2>/dev/null)
    babe_key=$(echo "$babe_output" | grep "SS58 Address" | awk '{print $3}')
    
    # Generar claves Ed25519 (GRANDPA)
    grandpa_output=$(./target/release/tokfin-node key inspect "//$validator" --scheme ed25519 2>/dev/null)
    grandpa_key=$(echo "$grandpa_output" | grep "SS58 Address" | awk '{print $3}')
    
    # Para account, usamos la dirección derivada de BABE (común en Substrate)
    account_output=$(./target/release/tokfin-node key inspect "//$validator" --scheme sr25519 2>/dev/null)
    account_hex=$(echo "$account_output" | grep "Account ID" | awk '{print $3}')
    
    if [[ -n "$babe_key" && -n "$grandpa_key" && -n "$account_hex" ]]; then
        echo "  account: $account_hex"
        echo "  babe:    $babe_key"  
        echo "  grandpa: $grandpa_key"
        
        # Almacenar en arrays
        ACCOUNTS[$i]="$account_hex"
        BABE_KEYS[$i]="$babe_key"
        GRANDPA_KEYS[$i]="$grandpa_key"
    else
        echo "❌ Error generando claves para $validator"
        exit 1
    fi
done

# 3. Modificar el JSON para incluir las claves en session
echo "🔍 Actualizando session keys en $SPEC_FILE..."

# Crear el JSON de session keys
SESSION_KEYS_JSON="["
for i in "${!VALIDATORS[@]}"; do
    if [[ $i -gt 0 ]]; then
        SESSION_KEYS_JSON+=","
    fi
    
    SESSION_KEYS_JSON+="[\"${ACCOUNTS[$i]}\",\"${ACCOUNTS[$i]}\",{\"babe\":\"${BABE_KEYS[$i]}\",\"grandpa\":\"${GRANDPA_KEYS[$i]}\"}]"
done
SESSION_KEYS_JSON+="]"

echo "🔍 Validando SESSION_KEYS_JSON..."
echo "Session keys generadas: ${#ACCOUNTS[@]} validadores"

# Usar jq para actualizar el JSON correctamente
if command -v jq >/dev/null 2>&1; then
    # Método seguro con jq
    echo "🔧 Actualizando JSON con jq..."
    
    # Crear archivo temporal con las session keys
    echo "$SESSION_KEYS_JSON" > /tmp/session_keys.json
    
    # Actualizar el chainspec
    jq --argjson keys "$(cat /tmp/session_keys.json)" \
       '.genesis.runtimeGenesis.patch.session.keys = $keys' \
       "$SPEC_FILE" > "${SPEC_FILE}.tmp"
    
    # Verificar que el resultado sea JSON válido
    if jq empty "${SPEC_FILE}.tmp" >/dev/null 2>&1; then
        mv "${SPEC_FILE}.tmp" "$SPEC_FILE"
        echo "✅ Session keys actualizadas correctamente"
    else
        echo "❌ Error actualizando session keys con jq"
        rm -f "${SPEC_FILE}.tmp" /tmp/session_keys.json
        exit 1
    fi
    
    rm -f /tmp/session_keys.json
else
    # Método alternativo sin jq (más frágil)
    echo "⚠️ jq no disponible, usando sed (menos seguro)"
    
    # Buscar y reemplazar la sección de session keys
    sed -i "s/\"keys\": \[[^]]*\]/\"keys\": $SESSION_KEYS_JSON/g" "$SPEC_FILE"
fi

echo "✅ Spec actualizado: $SPEC_FILE"

# Mostrar resumen
if command -v jq >/dev/null 2>&1; then
    echo ""
    echo "📋 Resumen del chainspec actualizado:"
    echo "  - Session keys: $(jq '.genesis.runtimeGenesis.patch.session.keys | length' "$SPEC_FILE")"
    echo "  - BABE authorities: $(jq '.genesis.runtimeGenesis.patch.babe.authorities | length' "$SPEC_FILE")"
    echo "  - GRANDPA authorities: $(jq '.genesis.runtimeGenesis.patch.grandpa.authorities | length' "$SPEC_FILE")"
fi

# 4. Generar raw spec con las claves incluidas
echo ""
echo "⛏️ Generando raw spec -> $RAW_FILE"

if ./target/release/tokfin-node build-spec --chain="$SPEC_FILE" --raw > "$RAW_FILE" 2>raw_spec_error.log; then
    echo "✅ Raw spec generado exitosamente: $RAW_FILE"
    rm -f raw_spec_error.log
else
    echo "❌ Error generando raw spec"
    echo ""
    echo "📝 Error detallado:"
    cat raw_spec_error.log
    echo ""
    echo "🔍 Verificando contenido del chainspec..."
    
    if command -v jq >/dev/null 2>&1; then
        if ! jq empty "$SPEC_FILE" >/dev/null 2>&1; then
            echo "❌ El chainspec contiene JSON inválido"
            exit 1
        else
            echo "✅ El chainspec es JSON válido"
        fi
    fi
    
    exit 1
fi

echo ""
echo "🎉 Script completado exitosamente"
echo "📄 Archivos generados:"
echo "  - Chainspec con session keys: $SPEC_FILE"
echo "  - Raw spec: $RAW_FILE"
echo ""
echo "👥 Validadores configurados:"
for i in "${!VALIDATORS[@]}"; do
    echo "  ${VALIDATORS[$i]}: ${ACCOUNTS[$i]}"
done