use std::collections::BTreeMap;

use crate::bytepair::BytePair;

pub trait VocabLoader {
    fn load(&self) -> BTreeMap<BytePair, u64>;
}
