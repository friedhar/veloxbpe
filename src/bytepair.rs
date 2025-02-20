pub(crate) const BYTEPAIR_CAPACITY: usize = 8;

#[derive(Ord, Eq, PartialEq, PartialOrd)]
pub struct BytePair {
    pub byte1: u8,
    pub byte2: Option<u8>,
}

impl BytePair {
    pub fn new_pair(byte1: u8, byte2: u8) -> BytePair {
        BytePair {
            byte1,
            byte2: Some(byte2),
        }
    }

    pub fn new_single(byte: u8) -> BytePair {
        BytePair {
            byte1: byte,
            byte2: None,
        }
    }
}
