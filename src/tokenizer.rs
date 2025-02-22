use std::mem;
use std::time::Instant;

use crate::smallstring::TinyString;
use crate::vocab::Vocab;
use pyo3::prelude::*;
use rayon::prelude::*;

#[pyclass]
pub struct BpeTokenizer {
    vocab: Vocab,
}

impl BpeTokenizer {
    pub fn new(vocab: Vocab) -> BpeTokenizer {
        BpeTokenizer { vocab }
    }
    pub fn encode(&self, x: &str) -> Vec<u64> {
        let s_t = Instant::now();

        let mut tokens: Vec<u64> = Vec::with_capacity(x.len());
        for c in x.chars() {
            tokens.push(match self.vocab.b2t.get(&TinyString::from_char(c)) {
                Some(z) => *z,
                None => continue,
            });
        }
        // let mut tokens: Vec<u64> = x
        //     .chars()
        //     .filter_map(|c| ))
        //     .map(|x| *x)
        //     .collect();
        dbg!(s_t.elapsed());
        let mut n = 0;
        let original_length = tokens.len();

        dbg!(&tokens);

        let s_t = Instant::now();
        dbg!(
            &self
                .vocab
                .t2b
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()[66]
        );
        loop {
            let mut new_tokens: Vec<u64> = Vec::with_capacity(tokens.len());
            let mut ix = 0;
            let mut modified = false;
            dbg!(&tokens);
            while ix + 1 < tokens.len() {
                let xs = [tokens[ix], tokens[ix + 1]];
                dbg!(xs);
                let ctx_left = &self.vocab.t2b[xs[0] as usize];
                let ctx_right = &self.vocab.t2b[xs[1] as usize];
                let ctx = TinyString::fuse(&ctx_left, &ctx_right);
                dbg!(ctx_left.to_string());
                dbg!(ctx_right.to_string());
                dbg!(ctx.to_string());
                match self.vocab.b2t.get(&ctx) {
                    Some(x) => {
                        ix += 2;
                        new_tokens.push(*x);
                        modified = true;
                    }
                    None => {
                        ix += 1;
                        new_tokens.push(xs[0]);
                    }
                };
            }

            n += 1;

            tokens = new_tokens;
            if !modified {
                break;
            }
        }
        // dbg!(s_t.elapsed());

        println!(
            "n: {n}, post length: {}, original_length: {original_length}, reduction: {:.2}%",
            tokens.len(),
            ((original_length - tokens.len()) as f64 / original_length as f64) * 100.0
        );

        tokens
    }
}

#[pymethods]
impl BpeTokenizer {
    pub fn py_encode(&self, x: &str) -> PyResult<Vec<u64>> {
        Ok(self.encode(x))
    }
}

#[cfg(test)]
mod tests {
    use std::{hint::black_box, time::Instant};

    use crate::{tokenizer::BpeTokenizer, vocab_loader::*};

    #[test]
    fn playground0() {
        let vocab: VocabLoader<O200kBase> = VocabLoader::<O200kBase>::new();
        let vocab = vocab.load().unwrap();
        let tokenizer = BpeTokenizer::new(vocab);
        dbg!(tokenizer.encode("dfdf"));
    }

    // #[test]
    // pub fn bench_bandwidth_encode() {
    //     // rayon::ThreadPoolBuilder::new()
    //     //     .num_threads(2)
    //     //     .build_global()
    //     //     .unwrap();

    //     let vocab: VocabLoader<O200kBase> = VocabLoader::<O200kBase>::new();
    //     let vocab = vocab.load().unwrap();

    //     let source: String = std::fs::read_to_string("./data/sample0.txt").unwrap();
    //     // let source = (&source[..10000]).to_string();
    //     let size = source.len();

    //     let tokenizer = BpeTokenizer::new(vocab);

    //     tokenizer.encode(&source);
    //     loop {
    //         let start_t = Instant::now();
    //         black_box(tokenizer.encode(&source));
    //         let took_s = start_t.elapsed().as_micros() as f64 / 1e6 as f64;

    //         println!("MB / s: {}", size as f64 / took_s / 1e6);
    //     }
    // }
}
