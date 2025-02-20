use std::collections::{BTreeMap, HashMap};

use bytepair::BytePair;
pub mod bytepair;
pub struct Tokenizer {
    vocab: Vocab,
}

pub struct Vocab {
    inner: BTreeMap<BytePair, u64>,
}

impl Tokenizer {
    pub fn new(vocab: Vocab) -> Tokenizer {
        Tokenizer { vocab }
    }

    pub fn encode(&self, x: &str) -> Vec<u8> {
        let mut o: Vec<u8> = Vec::with_capacity(x.len());
        let mut bytes: Vec<u8> = x.bytes().collect();

        let mut last_token = None;

        for xs in bytes.windows(2) {
            let first = xs[0];
            let second = xs[1];

            last_token = second;
        }

        o
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytepair_compare_0() {
        let bytepair = BytePair::from_slice(&[1, 2, 3]).unwrap();
        assert_eq!(bytepair.compare(&[1]), false);
        assert_eq!(bytepair.compare(&[1, 2, 3]), true);
        assert_eq!(bytepair.compare(&[1, 2, 3, 4, 5, 6, 7, 8, 9]), false);
    }
}
