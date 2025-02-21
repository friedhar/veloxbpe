pub mod bytepair;
pub mod vocab;
pub mod vocab_loader;

use bytepair::BytePair;
use vocab::Vocab;

pub struct Tokenizer {
    vocab: Vocab,
}

impl Tokenizer {
    pub fn new(vocab: Vocab) -> Tokenizer {
        Tokenizer { vocab }
    }

    pub fn encode(&self, x: &str) -> Vec<u64> {
        let mut o: Vec<u64> = Vec::with_capacity(x.len());
        let bytes: Vec<u8> = x.bytes().collect();

        let mut ix = 0;
        while ix < bytes.len() {
            let first = bytes[ix];
            let second = bytes[ix + 1];

            match self.vocab.get(&BytePair::new_pair(first, second)) {
                Some(x) => {
                    ix += 2;
                    o.push(*x);
                }
                None => match self.vocab.get(&BytePair::new_single(first)) {
                    Some(x) => {
                        ix += 1;
                        o.push(*x);
                    }
                    None => todo!("return err"),
                },
            }
        }

        o
    }
}
