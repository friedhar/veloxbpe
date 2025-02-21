use std::collections::BTreeMap;

use crate::{bytepair::BytePair, vocab::Vocab};

pub trait VocabLoader {
    fn load(&self) -> Vocab;
}
