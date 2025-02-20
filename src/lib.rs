pub struct Tokenizer {
    vocab: Vocab,
}
pub struct Vocab {
    inner: (),
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
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
