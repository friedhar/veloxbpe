pub(crate) const BYTEPAIR_CAPACITY: usize = 8;

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
        if x.len() != self.length {
            return false;
        }

        for ix in 0..x.len() {
            if x[ix] != self.inner[ix] {
                dbg!(&self.inner);
                return false;
            }
        }
        true
    }
}
