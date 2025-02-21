use crate::{bytepair::BytePair, smallstring::SmallString};
use std::{borrow::Cow, collections::BTreeMap};

pub type Vocab = BTreeMap<String, u64>;
