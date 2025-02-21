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
        let mut n = 0;

        loop {
            let mut new_tokens: Vec<u64> = Vec::with_capacity(tokens.len() / 2);
            let mut ix = 0;
            let mut modified = false;
            while ix + 1 < tokens.len() {
                let xs = [tokens[ix], tokens[ix + 1]];
                let ctx_left = self.vocab.t2b[xs[0] as usize].clone();
                let ctx_right = self.vocab.t2b[xs[0] as usize].clone();
                let ctx = format!("{}{}", ctx_left.to_string(), ctx_right.to_string());
                match self.vocab.b2t.get(&SmartString::new(&ctx)) {
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
        // let source = (&source[..2048]).to_string();
        let size = source.len();

        let tokenizer = Tokenizer::new(vocab);
        let start_t = Instant::now();
        black_box(tokenizer.encode(&source));
        let took_s = start_t.elapsed().as_millis() as f64 / 1000.0;

        println!("MB / s: {}", size as f64 / took_s / 1e6);
    }
}
