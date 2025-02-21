use std::hash::{Hash, Hasher};

pub const SMALLSTRING_CAPACITY: usize = 128;

#[derive(Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct TinyString {
    inner: [u8; SMALLSTRING_CAPACITY],
    length: usize,
}

impl ToString for TinyString {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(
            (&self.inner[..self.length])
                .into_iter()
                .map(|x| *x as u8)
                .collect::<Vec<u8>>()
                .as_slice(),
        )
        .to_string()
    }
}

impl TinyString {
    pub fn new(s: &str) -> TinyString {
        if s.len() > SMALLSTRING_CAPACITY {
            todo!("retrun Err()");
        }
        let mut inner = [0; SMALLSTRING_CAPACITY];
        for (ix, i) in s.bytes().enumerate() {
            inner[ix] = i;
        }
        TinyString {
            inner,
            length: s.len(),
        }
    }

    pub fn fuse(a: &TinyString, b: &TinyString) -> TinyString {
        let mut inner = [0; SMALLSTRING_CAPACITY];
        for (ix, i) in (&a.inner[..a.length]).into_iter().enumerate() {
            inner[ix] = *i;
        }

        for (ix, i) in (&b.inner[..b.length]).into_iter().enumerate() {
            inner[a.length + ix] = *i;
        }

        TinyString {
            inner,
            length: a.length + b.length,
        }
    }

    pub fn from_char(c: char) -> TinyString {
        TinyString::new(c.to_string().as_str())
    }
}

impl Hash for TinyString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.inner); // Only hashing `id` for simplicity
    }
}
