use serde::{Deserialize, Serialize};

const SMALLSTRING_CAPACITY: usize = 32;

#[derive(Ord, Eq, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct SmallString {
    inner: [char; SMALLSTRING_CAPACITY],
    length: usize,
}

impl SmallString {
    pub fn new(s: &str) -> SmallString {
        if s.len() > SMALLSTRING_CAPACITY {
            todo!("retrun Err()");
        }
        let mut inner = ['\0'; SMALLSTRING_CAPACITY];
        for (ix, i) in s.chars().enumerate() {
            inner[ix] = i;
        }
        SmallString {
            inner,
            length: s.len(),
        }
    }
}
