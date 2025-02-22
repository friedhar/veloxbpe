use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::smallstring::TinyString;
use std::collections::HashMap;

pub type Bytes2Token = HashMap<TinyString, u64>;

#[derive(Deserialize, Serialize, Clone)]
pub struct VocabIntermidiate {
    pub b2t: HashMap<String, u64>,
    pub t2b: HashMap<u64, String>,
}

impl VocabIntermidiate {
    pub(crate) fn to_real(&self) -> Vocab {
        Vocab {
            b2t: self
                .b2t
                .iter()
                .map(|(k, v)| (TinyString::new(k), *v))
                .collect(),
            t2b: self
                .t2b
                .iter()
                .map(|(k, v)| (*k, TinyString::new(&v)))
                .collect(),
        }
    }
}

#[derive(Clone)]
#[pyclass]
pub struct Vocab {
    pub b2t: HashMap<TinyString, u64>,
    pub t2b: HashMap<u64, TinyString>,
}

impl Vocab {
    pub fn new(x: Bytes2Token) -> Vocab {
        let length = x.clone().into_values().max().unwrap() + 1;
        dbg!(length);
        // let mut t2b: Vec<Option<TinyString>> = vec![None; length as usize];
        // for (k, v) in x.iter() {
        //     // dbg!(v);
        //     t2b[*v as usize] = Some(k.clone());
        // }
        // let t2b: Vec<TinyString> = t2b.into_iter().filter_map(|x| x).collect();
        let t2b = x.iter().map(|(k, v)| (*v, *k)).collect();

        Vocab { b2t: x, t2b }
    }

    pub(crate) fn to_intermidiate(&self) -> VocabIntermidiate {
        VocabIntermidiate {
            b2t: self.b2t.iter().map(|(k, v)| (k.to_string(), *v)).collect(),
            t2b: self.t2b.iter().map(|(k, v)| (*k, v.to_string())).collect(),
        }
    }
}
