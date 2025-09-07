#!/usr/bin/env python3

from decimal import Decimal

DECIMALS = 18  # 18 decimales para TKF/TKFr/TKFe
UNIT = 10 ** DECIMALS

def to_plancks(amount_tkf: float) -> int:
    return int(Decimal(amount_tkf) * UNIT)

def from_plancks(amount_plancks: int) -> Decimal:
    return Decimal(amount_plancks) / UNIT

if __name__ == "__main__":
    # Ejemplos
    print("1 TKF   =", to_plancks(1), "plancks")
    print("10 TKF  =", to_plancks(10), "plancks")
    print("1M TKF  =", to_plancks(1_000_000), "plancks")
    print("4M TKF  =", to_plancks(4_000_000), "plancks")

    # Conversión inversa
    print("4000000000000000000000000 plancks =", from_plancks(4000000000000000000000000), "TKF")
