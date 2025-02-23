use crate::smallstring::TinyString;
use crate::vocab::Vocab;
use pyo3::prelude::*;
use rayon::{prelude::*, ThreadPool, ThreadPoolBuilder};

#[pyclass]
pub struct Tokenizer {
    vocab: Vocab,
    pool: ThreadPool,
}

impl Tokenizer {
    pub fn new(vocab: Vocab, threads: usize) -> Tokenizer {
        Tokenizer {
            vocab,
            pool: ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap(),
        }
    }

    pub fn encode(&self, x: &str) -> Vec<u64> {
        x.split(" ")
            .map(|x| Tokenizer::encode_l0(&self, &x))
            .flatten()
            .collect()
    }

    pub fn encode_batch(&self, x: &[String]) -> Vec<u64> {
        self.pool.install(|| {
            x.into_par_iter()
                .map(|x| self.encode(x))
                .flatten()
                .collect()
        })
    }
    pub fn encode_l0(&self, x: &str) -> Vec<u64> {
        let mut tokens: Vec<u64> = Vec::with_capacity(x.len());
        let mut i = 0;

        let cs: Vec<char> = x.chars().collect();

        while i < cs.len() {
            let mut best_len = 1;
            let mut best_token = 0;

            for j in i + 1..cs.len() - 1 {
                let subtxt = &cs[i..j + 1];
                let length_i = j - i; // always > 0
                if length_i >= self.vocab.max_word_len {
                    break;
                }

                match self.vocab.b2t.get(&TinyString::from_chars(subtxt)) {
                    Some(tid) => {
                        // println!("new best :: {i} tid: {tid}, length: {length_i}");
                        best_len = length_i;
                        best_token = *tid;
                    }
                    None => {}
                }
            }

            if best_len > 1 {
                i += best_len + 1;
            } else {
                best_token = match self.vocab.b2t.get(&TinyString::from_char(cs[i])) {
                    Some(x) => *x,
                    None => {
                        i += 1;
                        continue;
                    }
                };
                i += 1;
            }

            tokens.push(best_token);
        }

        tokens
    }
}

#[pymethods]
impl Tokenizer {
    pub fn py_encode(&self, x: &str) -> PyResult<Vec<u64>> {
        Ok(self.encode(x))
    }
}

#[cfg(test)]
mod tests {

    use std::{hint::black_box, time::Instant};

    use crate::{tokenizer::Tokenizer, vocab_loader::*};

    #[test]
    fn playground0() {
        let vocab: VocabLoader<O200kBase> = VocabLoader::<O200kBase>::new();
        let vocab = vocab.load().unwrap();
        let tokenizer = Tokenizer::new(vocab, 1);
        dbg!(tokenizer.encode("dfdf"));
    }

    #[test]
    pub fn bench_bandwidth_encode() {
        // rayon::ThreadPoolBuilder::new()
        //     .num_threads(2)
        //     .build_global()
        //     .unwrap();

        let vocab: VocabLoader<O200kBase> = VocabLoader::<O200kBase>::new();
        let vocab = vocab.load().unwrap();

        let source: String = std::fs::read_to_string("./data/sample0.txt").unwrap();
        // let source = (&source[..10000]).to_string();
        let size = source.len();

        let tokenizer = Tokenizer::new(vocab, 1);

        tokenizer.encode(&source);
        {
            let start_t = Instant::now();
            black_box(tokenizer.encode(&source));
            let took_s = start_t.elapsed().as_micros() as f64 / 1e6 as f64;

            println!("MB / s: {}", size as f64 / took_s / 1e6);
        }
    }
}
