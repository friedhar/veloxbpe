use std::collections::BTreeMap;

use crate::{bytepair::BytePair, vocab::Vocab};

pub trait VocabLoader {
    fn load(&self) -> Vocab;
}

pub struct O200kBase {}
impl VocabLoader for O200kBase {
    fn load(&self) -> Vocab {}
}
