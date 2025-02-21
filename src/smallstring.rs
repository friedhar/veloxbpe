use serde::{Deserialize, Serialize};

const SMALLSTRING_CAPACITY: usize = 32;

#[derive(Ord, Eq, PartialEq, PartialOrd, Deserialize, Serialize, Clone)]
pub enum SmartString {
    Stack(TinyString),
    Heap(String),
}

impl SmartString {
    pub fn new(s: &str) -> SmartString {
        match s.len() > SMALLSTRING_CAPACITY {
            true => SmartString::Heap(s.to_string()),
            false => SmartString::Stack(TinyString::new(s)),
        }
    }

    pub fn from_char(c: char) -> SmartString {
        SmartString::Stack(TinyString::new(c.to_string().as_str()))
    }
}

impl ToString for SmartString {
    fn to_string(&self) -> String {
        use SmartString::*;
        match self {
            Stack(x) => x.to_string(),
            Heap(x) => x.to_string(),
        }
    }
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Deserialize, Serialize, Clone)]
pub struct TinyString {
    inner: [char; SMALLSTRING_CAPACITY],
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
        let mut inner = ['\0'; SMALLSTRING_CAPACITY];
        for (ix, i) in s.chars().enumerate() {
            inner[ix] = i;
        }
        TinyString {
            inner,
            length: s.len(),
        }
    }
}
