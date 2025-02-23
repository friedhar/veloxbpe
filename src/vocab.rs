use pyo3::prelude::*;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

use crate::smallstring::TinyString;
use std::collections::HashMap;

pub type Bytes2Token = FxHashMap<TinyString, u64>;

#[derive(Deserialize, Serialize, Clone)]
pub struct VocabIntermidiate {
    pub b2t: HashMap<String, u64>,
}

impl VocabIntermidiate {
    pub(crate) fn to_real(&self) -> Vocab {
        Vocab {
            b2t: self
                .b2t
                .iter()
                .map(|(k, v)| (TinyString::new(k), *v))
                .collect(),

            max_word_len: self.b2t.keys().map(|x| x.len()).max().unwrap(),
        }
    }
}

#[derive(Clone)]
#[pyclass]
pub struct Vocab {
    pub b2t: FxHashMap<TinyString, u64>,
    pub max_word_len: usize,
}

impl Vocab {
    pub fn new(x: Bytes2Token) -> Vocab {
        let length = x.clone().into_values().max().unwrap() + 1;
        dbg!(length);
        Vocab {
            b2t: x,
            max_word_len: 0,
        }
    }

    pub(crate) fn to_intermidiate(&self) -> VocabIntermidiate {
        VocabIntermidiate {
            b2t: self.b2t.iter().map(|(k, v)| (k.to_string(), *v)).collect(),
        }
    }
}
