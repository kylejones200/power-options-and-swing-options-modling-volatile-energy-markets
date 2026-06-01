use power_options_and_swing_options_modling_volatile_energy_markets_core::call_intrinsic_values;

fn main() {
    let spots: Vec<f64> = (0..5000).map(|i| 40.0 + (i % 50) as f64).collect();
    for _ in 0..10_000 {
        let _ = call_intrinsic_values(&spots, 45.0);
    }
}
