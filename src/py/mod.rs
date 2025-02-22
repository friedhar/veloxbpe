use pyo3::{
    exceptions::{asyncio::CancelledError, PyRuntimeError},
    prelude::*,
};

use crate::{
    tokenizer::BpeTokenizer,
    vocab::Vocab,
    vocab_loader::{O200kBase, VocabLoader},
};

#[pyclass]
pub struct PyTokenizer {
    x: BpeTokenizer,
}

#[pymethods]
impl PyTokenizer {
    #[new]
    pub fn new() -> PyResult<PyTokenizer> {
        let vocab: VocabLoader<O200kBase> = VocabLoader::new();
        let vocab = match vocab.load() {
            Ok(x) => x,
            Err(_) => return Err(PyErr::new::<PyRuntimeError, _>("")),
        };
        Ok(PyTokenizer {
            x: BpeTokenizer::new(vocab),
        })
    }

    pub fn encode(&self, x: &str) -> PyResult<Vec<u64>> {
        Ok(self.x.encode(x))
    }
}

// #[pyfunction]
// fn tokenizer_for_vocab(vocab_name: &str) -> PyResult<BpeTokenizer> {
//     println!("vocab_name: {vocab_name}");
//     let vocab: VocabLoader<O200kBase> = VocabLoader::new();
//     let vocab = vocab.load().unwrap();
//     Ok(BpeTokenizer::new(vocab))
// }

#[pymodule]
fn veloxbpe(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(tokenizer_for_vocab, m)?)?;
    Ok(())
}
