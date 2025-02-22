use bpe::BpeTokenizer;
use pyo3::prelude::*;
use vocab_loader::{O200kBase, VocabLoader};

pub mod base64;
pub mod bpe;
pub mod bpe_worker;
pub mod bytepair;
pub mod smallstring;
pub mod vocab;
pub mod vocab_loader;

#[pyfunction]
fn tokenizer() -> PyResult<BpeTokenizer> {
    let vocab: VocabLoader<O200kBase> = VocabLoader::new();
    let vocab = vocab.load().unwrap();
    Ok(BpeTokenizer::new(vocab))
}

// #[pymodule]

#[pymodule]
fn veloxbpe(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenizer, m)?)?;
    Ok(())
}
