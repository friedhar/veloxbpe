use crate::{bytepair::BytePair, smallstring::SmallString};
use std::collections::BTreeMap;

pub type Vocab = BTreeMap<SmallString, u64>;
