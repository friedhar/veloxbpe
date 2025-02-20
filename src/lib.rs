use std::collections::HashMap;

const BYTEPAIR_CAPACITY: usize = 8;

pub struct Tokenizer {
    vocab: Vocab,
}

pub struct Vocab {
    inner: HashMap<Vec<u8>, u64>,
}

pub struct BytePair {
    inner: [u8; BYTEPAIR_CAPACITY],
    length: usize,
}

impl BytePair {
    pub fn from_slice(x: &[u8]) -> Option<BytePair> {
        if x.len() > BYTEPAIR_CAPACITY {
            return None;
        }
        let mut inner = [0; BYTEPAIR_CAPACITY];
        for (ix, i) in x.into_iter().enumerate() {
            inner[ix] = *i; // u8 is cheap to copy..
        }

        Some(BytePair {
            inner,
            length: x.len(),
        })
    }

    pub fn compare(&self, x: &[u8]) -> bool {
        if x.len() != BYTEPAIR_CAPACITY {
            return false;
        }

        for ix in 0..x.len() {
            if x[ix] != self.inner[ix] {
                return false;
            }
        }
        true
    }
}

impl Tokenizer {
    pub fn new(vocab: Vocab) -> Tokenizer {
        Tokenizer { vocab }
    }

    pub fn encode(&self, x: &str) -> Vec<u8> {
        let mut o: Vec<u8> = Vec::with_capacity(x.len());
        let bytes = x.bytes();

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
