#!/usr/bin/env python3
"""Python vs Rust kernel benchmark."""

from __future__ import annotations

import time
import sys
from pathlib import Path

import numpy as np

ROOT = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT / "src"))
from compute_kernel import call_intrinsic_values  # noqa: E402

def main() -> None:
    spots = np.ascontiguousarray(40.0 + np.arange(5000) % 50, dtype=float)
    strike = 45.0
    t0 = time.perf_counter()
    for _ in range(200):
        call_intrinsic_values(spots, strike)
    py_s = time.perf_counter() - t0
    try:
        import power_options_and_swing_options_modling_volatile_energy_markets_rs as rs
    except ImportError:
        print("Build: maturin develop --release -m rust/py/Cargo.toml")
        print(f"Python {py_s:.3f}s")
        return
    rs_s = rs.bench_kernel_py(spots, strike, 10000)
    print(f"Python {py_s:.3f}s Rust {rs_s:.3f}s speedup {py_s / max(rs_s, 1e-9):.1f}x")
    np.testing.assert_allclose(
        call_intrinsic_values(spots, strike),
        np.asarray(rs.call_intrinsic_values_py(spots, strike)),
        rtol=1e-12,
    )
    print("Correctness: OK")

if __name__ == "__main__":
    main()
