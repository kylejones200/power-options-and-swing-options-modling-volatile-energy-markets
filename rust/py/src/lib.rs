use power_options_and_swing_options_modling_volatile_energy_markets_core::call_intrinsic_values;
use numpy::{PyArray1, PyReadonlyArray1, IntoPyArray};
use pyo3::prelude::*;

#[pyfunction]
fn call_intrinsic_values_py<'py>(py: Python<'py>, spots: PyReadonlyArray1<f64>, strike: f64) -> PyResult<Bound<'py, PyArray1<f64>>> {
    Ok(call_intrinsic_values(spots.as_slice()?, strike).into_pyarray(py))
}

#[pyfunction]
#[pyo3(signature = (spots, strike, iterations=500))]
fn bench_kernel_py(spots: PyReadonlyArray1<f64>, strike: f64, iterations: usize) -> PyResult<f64> {
    let spots_buf = spots.as_slice()?.to_vec();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = call_intrinsic_values(&spots_buf, strike);
    }
    Ok(start.elapsed().as_secs_f64())
}

#[pymodule]
fn power_options_and_swing_options_modling_volatile_energy_markets_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(call_intrinsic_values_py, m)?)?;
    m.add_function(wrap_pyfunction!(bench_kernel_py, m)?)?;
    Ok(())
}
