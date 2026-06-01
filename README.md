# Power Options and Swing Options Modling Volatile Energy Markets

Published: 2025-10-06
Medium: [https://medium.com/@kyle-t-jones/power-options-and-swing-options-modling-volatile-energy-markets-383f332f132b](https://medium.com/@kyle-t-jones/power-options-and-swing-options-modling-volatile-energy-markets-383f332f132b)

## Business context

When Hurricane Ida shut down Louisiana's power grid in August 2021, electricity prices in neighboring markets spiked to $1,500/MWh. Traders holding call options on power made fortunes --- their $8/MWh premiums turned into $1,400/MWh payoffs overnight. Meanwhile, generators locked into fixed-price forward contracts faced catastrophic losses, watching spot prices soar 1,800% above their contracted rates.

Power options are sophisticated instruments that monetize volatility, hedge price risk, and create leverage that amplifies gains while limiting losses.

Power markets exhibit extreme volatility compared to other commodities. Oil prices might move 3--5% daily; power prices routinely swing 50--200% within hours. This volatility creates both enormous risk and extraordinary opportunity.



## Rust performance port

Side-by-side **Python vs Rust** implementation of the numeric hot loop — European call intrinsic values. Reference PyO3 benchmark: **see `benchmark_rust.py`** on a release build (local machine; run `benchmark_rust.py` to reproduce).

| Path | Role |
|------|------|
| `src/compute_kernel.py` | Python/numpy reference kernel |
| `rust/core/` | Pure Rust library |
| `rust/py/` | PyO3 bindings |
| `rust/bench/` | Standalone CLI benchmark |
| `benchmark_rust.py` | Python vs Rust timing + correctness check |

```bash
# Rust-only CLI benchmark
cd rust && cargo run --release -p power_options_and_swing_options_modling_volatile_energy_markets_bench

# Python vs Rust (PyO3)
pip install maturin numpy
maturin develop --release -m rust/py/Cargo.toml
python benchmark_rust.py
```

Python ML training, solvers, and orchestration stay in Python; Rust targets the numeric hot loops. Stochastic generators validate output shapes; deterministic kernels match at tight floating-point tolerance.


## Disclaimer

Educational/demo code only. Not financial, safety, or engineering advice. Use at your own risk. Verify results independently before any production or operational use.

## License

MIT — see [LICENSE](LICENSE).