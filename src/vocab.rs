use serde::{Deserialize, Serialize};
use serde_json::StreamDeserializer;

use crate::{
    bytepair::BytePair,
    smallstring::{SmartString, TinyString},
};
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
};

pub type Bytes2Token = HashMap<TinyString, u64>;

#[derive(Deserialize, Serialize, Clone)]
pub struct VocabIntermidiate {
    pub b2t: BTreeMap<String, u64>,
    pub t2b: Vec<String>,
}

impl VocabIntermidiate {
    pub(crate) fn to_real(&self) -> Vocab {
        Vocab {
            b2t: self
                .b2t
                .iter()
                .map(|(k, v)| (TinyString::new(k), *v))
                .collect(),
            t2b: self.t2b.iter().map(|x| TinyString::new(&x)).collect(),
        }
    }
}

#[derive(Clone)]
pub struct Vocab {
    pub b2t: HashMap<TinyString, u64>,
    pub t2b: Vec<TinyString>,
}

impl Vocab {
    pub fn new(x: Bytes2Token) -> Vocab {
        let length = x.clone().into_values().max().unwrap();
        dbg!(length);
        let mut t2b: Vec<TinyString> = Vec::with_capacity(length as usize);
        for (k, _) in x.iter() {
            t2b.push(k.clone());
        }

        Vocab { b2t: x, t2b }
    }

    pub(crate) fn to_intermidiate(&self) -> VocabIntermidiate {
        VocabIntermidiate {
            b2t: self.b2t.iter().map(|(k, v)| (k.to_string(), *v)).collect(),
            t2b: self.t2b.iter().map(|x| x.to_string()).collect(),
        }
    }
}
