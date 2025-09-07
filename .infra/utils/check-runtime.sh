
---

### 📄 `/.infra/utils/check-runtime.sh`

```bash
#!/usr/bin/env bash
set -e

BIN=${1:-./target/release/tokfin-node}
CHAIN=${2:-./chain-specs/tokfin.json}

echo "🔍 Checking runtime hash for $BIN ..."
$BIN inspect-node --chain $CHAIN | grep -A3 "Runtime"

