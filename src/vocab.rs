use std::collections::BTreeMap;

use crate::bytepair::BytePair;

pub struct Vocab {
    inner: BTreeMap<BytePair, u64>,
}
