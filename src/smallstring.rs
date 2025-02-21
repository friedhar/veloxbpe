const SMALLSTRING_CAPACITY: usize = 8;

pub struct SmallString {
    inner: [char; 8],
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
