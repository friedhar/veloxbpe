use crate::{
    bytepair::BytePair,
    smallstring::{SmartString, TinyString},
};
use std::{borrow::Cow, collections::BTreeMap};

pub type Vocab = BTreeMap<SmartString, u64>;
