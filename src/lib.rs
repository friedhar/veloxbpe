pub mod base64;
pub mod bytepair;
pub mod smallstring;
pub mod vocab;
pub mod vocab_loader;

use bytepair::BytePair;
use rayon::prelude::*;
use smallstring::SmartString;
use vocab::{Bytes2Token, Vocab};

pub struct Tokenizer {
    vocab: Vocab,
}

impl Tokenizer {
    pub fn new(vocab: Vocab) -> Tokenizer {
        Tokenizer { vocab }
    }

    pub fn encode(&self, x: &str) -> Vec<u64> {
        // let mut o: Vec<u64> = Vec::with_capacity(x.len());
        // let bytes: Vec<u8> = x.bytes().collect();
        let mut tokens: Vec<u64> = x
            .chars()
            .filter_map(|c| self.vocab.b2t.get(&SmartString::from_char(c)))
            .map(|x| *x)
            .collect();

        loop {
            // for xs in tokens.windows(2) {
            //     let ctx_left = vocab.xs[0];
            // }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use std::{hint::black_box, time::Instant};

    use rayon::iter::IntoParallelRefIterator;

    use crate::{vocab_loader::*, Tokenizer};

    #[test]
    fn playground0() {
        let vocab: VocabLoader<O200kBase> = VocabLoader::<O200kBase>::new();
        let vocab = vocab.load().unwrap();
        let tokenizer = Tokenizer::new(vocab);
        dbg!(tokenizer.encode("dfdf"));
    }

    #[test]
    pub fn bench_bandwidth_encode() {
        rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build_global()
            .unwrap();
        let vocab: VocabLoader<O200kBase> = VocabLoader::<O200kBase>::new();
        let vocab = vocab.load().unwrap();

        let source: String = vocab.b2t.iter().map(|(k, _)| k.to_string()).collect();
        let size = source.len();

        let tokenizer = Tokenizer::new(vocab);
        let start_t = Instant::now();
        black_box(tokenizer.encode(&source));
        let took_s = start_t.elapsed().as_millis() as f64 / 1000.0;

        println!("MB / s: {}", size as f64 / took_s / 1e6);
    }
}
