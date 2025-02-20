use std::collections::{BTreeMap, HashMap};

use bytepair::BytePair;

pub mod bytepair;
pub mod vocab;
pub mod vocab_loader;

pub struct Tokenizer {
    vocab: BTreeMap<BytePair, u64>,
}

impl Tokenizer {
    pub fn new(vocab: BTreeMap<BytePair, u64>) -> Tokenizer {
        Tokenizer { vocab }
    }

    pub fn encode(&self, x: &str) -> Vec<u64> {
        let mut o: Vec<u64> = Vec::with_capacity(x.len());
        let bytes: Vec<u8> = x.bytes().collect();

        let mut ix = 0;
        while ix < bytes.len() {
            let first = bytes[ix];
            let second = bytes[ix + 1];

            let pair = self.vocab.get(&BytePair::new_pair(first, second));
            if let Some(x) = pair {
                ix += 2;
                o.push(*x);
            }
        }

        o
    }
}
