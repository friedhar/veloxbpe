use pyo3::{pyfunction, PyResult};

pub mod base64;
pub mod bpe;
pub mod bpe_worker;
pub mod bytepair;
pub mod smallstring;
pub mod vocab;
pub mod vocab_loader;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
