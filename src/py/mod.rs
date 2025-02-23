use pyo3::{exceptions::PyRuntimeError, prelude::*};

use crate::{
    tokenizer::Tokenizer,
    vocab_loader::{O200kBase, VocabLoader},
};

#[pyclass(name = "Tokenizer")]
pub struct PyTokenizer {
    x: Tokenizer,
}

#[pymethods]
impl PyTokenizer {
    #[new]
    pub fn new(vocab_name: &str, threads: usize) -> PyResult<PyTokenizer> {
        let vocab = match vocab_name {
            "" | "o200k_base" => {
                let vocab: VocabLoader<O200kBase> = VocabLoader::new();
                let vocab = match vocab.load() {
                    Ok(x) => x,
                    Err(_) => return Err(PyErr::new::<PyRuntimeError, _>("")),
                };
                vocab
            }
            _ => return Err(PyErr::new::<PyRuntimeError, _>("Vocabulary doesn't exist.")),
        };
        Ok(PyTokenizer {
            x: Tokenizer::new(vocab, threads),
        })
    }

    pub fn encode(&self, x: &str) -> PyResult<Vec<u64>> {
        Ok(self.x.encode(x))
    }

    pub fn encode_batch(&self, x: Vec<String>) -> PyResult<Vec<u64>> {
        Ok(self.x.encode_batch(&x))
    }
}

#[pymodule]
fn veloxbpe(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTokenizer>()?;
    Ok(())
}
