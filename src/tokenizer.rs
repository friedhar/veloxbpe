use std::str::MatchIndices;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use crate::smallstring::TinyString;
use crate::vocab::Vocab;
use pyo3::ffi::newfunc;
use pyo3::{prelude::*, PyErrArguments};
use rayon::{prelude::*, ThreadBuilder, ThreadPool, ThreadPoolBuilder};

#[pyclass]
pub struct BpeTokenizer {
    vocab: Vocab,
    workers: usize,
}

impl BpeTokenizer {
    pub fn new(vocab: Vocab, threads: usize) -> BpeTokenizer {
        BpeTokenizer {
            vocab,
            workers: threads,
        }
    }

    pub fn encode(&self, x: &str) -> Vec<u64> {
        let xs: Vec<String> = x.split(" ").map(|x| x.to_string()).collect();
        // let o: Vec<u64> = self
        //     .pool
        //     .install(|| xs.map(|x| self.encode_l0(x)).flatten().collect());

        let batch_size = xs.len() / self.workers;
        let remain = xs.len() % self.workers; // gotta take care of leftover..

        let o = crossbeam::scope(|scope| {
            let mut handles = Vec::with_capacity(self.workers);
            for worker in 0..self.workers {
                let batch = (xs[worker * batch_size..worker * batch_size + batch_size]).to_vec();
                handles.push(scope.spawn(|_| {
                    batch
                        .into_iter()
                        .map(|x| BpeTokenizer::encode_l0(&self, &x))
                        .flatten()
                        .collect()
                }));
            }

            let mut o = Vec::new();
            for handle in handles.into_iter() {
                let o_i: Vec<u64> = handle.join().unwrap();
                o.extend(&o_i);
            }
            o
        })
        .unwrap();

        o
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
                // println!("best best, i: {i}, start_char: {}", &x[i - 3..]);
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

            // dbg!(&tokens
            //     .iter()
            //     .map(|x| self.vocab.t2b.get(&x).unwrap().to_string())
            //     .collect::<Vec<String>>());
        }

        tokens
    }

    // pub fn encode_old(&self, x: &str) -> Vec<u64> {
    //     let mut tokens: Vec<u64> = Vec::with_capacity(x.len());
    //     for c in x.chars() {
    //         tokens.push(match self.vocab.b2t.get(&TinyString::from_char(c)) {
    //             Some(z) => *z,
    //             None => continue,
    //         });
    //     }

    //     let mut n = 0;

    //     let batch_size = tokens.len() / self.n_workers;
    //     let batch_size = if batch_size > 0 {
    //         batch_size
    //     } else {
    //         tokens.len()
    //     };

    //     // for i in 0..tokens.len() {
    //     //     for j in i + 1..tokens {}
    //     // }

    //     loop {
    //         if tokens.len() == 1 {
    //             break;
    //         }
    //         let mut new_tokens: Vec<u64> = Vec::with_capacity(tokens.len());
    //         let mut ix = 0;
    //         let mut modified = false;
    // dbg!(&tokens
    //     .iter()
    //     .map(|x| self.vocab.t2b.get(&x).unwrap().to_string())
    //     .collect::<Vec<String>>());
    //         // dbg!(&tokens);
    //         while ix < tokens.len() {
    //             if ix + 1 >= tokens.len() {
    //                 new_tokens.push(tokens[ix]);
    //                 break;
    //             }
    //             let xs = [tokens[ix], tokens[ix + 1]];

    //             let ctx_left = &self.vocab.t2b_seq.get(xs[0] as usize).unwrap().unwrap();
    //             let ctx_right = &self.vocab.t2b_seq.get(xs[1] as usize).unwrap().unwrap();
    //             let ctx = TinyString::fuse(&ctx_left, &ctx_right);
    //             match self.vocab.b2t.get(&ctx) {
    //                 Some(x) => {
    //                     ix += 2;
    //                     new_tokens.push(*x);
    //                     modified = true;
    //                 }
    //                 None => {
    //                     ix += 1;
    //                     new_tokens.push(xs[0]);
    //                 }
    //             };
    //         }

    //         n += 1;

    //         tokens = new_tokens;
    //         if !modified {
    //             break;
    //         }
    //     }

    //     // println!(
    //     //     "n: {n}, post length: {}, original_length: {original_length}, reduction: {:.2}%",
    //     //     tokens.len(),
    //     //     ((original_length - tokens.len()) as f64 / original_length as f64) * 100.0
    //     // );

    //     tokens
    // }
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
        let tokenizer = BpeTokenizer::new(vocab, 1);
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

        let tokenizer = BpeTokenizer::new(vocab, 1);

        tokenizer.encode(&source);
        {
            let start_t = Instant::now();
            black_box(tokenizer.encode(&source));
            let took_s = start_t.elapsed().as_micros() as f64 / 1e6 as f64;

            println!("MB / s: {}", size as f64 / took_s / 1e6);
        }
    }
}
