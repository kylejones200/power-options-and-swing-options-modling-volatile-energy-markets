//! European call intrinsic value batch.

pub fn call_intrinsic_values(spots: &[f64], strike: f64) -> Vec<f64> {
    spots.iter().map(|&s| (s - strike).max(0.0)).collect()
}
