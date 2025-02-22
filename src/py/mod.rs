use pyo3::prelude::*;

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
