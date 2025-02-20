use std::collections::{BTreeMap, HashMap};

use bytepair::BytePair;
use vocab::Vocab;

pub mod bytepair;
pub mod vocab;

pub struct Tokenizer {
    vocab: BTreeMap<BytePair, u64>,
}

impl Tokenizer {
    pub fn new(vocab: BTreeMap<BytePair, u64>) -> Tokenizer {
        Tokenizer { vocab }
    }

    pub fn encode(&self, x: &str) -> Vec<u8> {
        let mut o: Vec<u8> = Vec::with_capacity(x.len());
        let mut bytes: Vec<u8> = x.bytes().collect();

        for xs in bytes.windows(2) {
            let first = xs[0];
            let second = xs[1];

            let pair = self.vocab.get(&BytePair::new_pair(first, second));
        }

        o
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
