use serde::{Deserialize, Serialize};

use crate::{
    bytepair::BytePair,
    smallstring::{SmartString, TinyString},
};
use std::{borrow::Cow, collections::BTreeMap};

pub type Bytes2Token = BTreeMap<SmartString, u64>;

#[derive(Deserialize, Serialize, Clone)]
pub struct Vocab {
    pub b2t: BTreeMap<SmartString, u64>,
    pub t2b: Vec<SmartString>,
}

impl Vocab {
    pub fn new(x: Bytes2Token) -> Vocab {
        let length = x.clone().into_values().max().unwrap();
        dbg!(length);
        Vocab {
            b2t: x,
            t2b: vec![],
        }
    }
}
