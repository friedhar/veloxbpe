use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::smallstring::TinyString;
use std::{
    collections::{BTreeMap, HashMap},
    mem::MaybeUninit,
};

pub type Bytes2Token = BTreeMap<TinyString, u64>;

#[derive(Deserialize, Serialize, Clone)]
pub struct VocabIntermidiate {
    pub b2t: BTreeMap<String, u64>,
    pub t2b: Vec<String>,
}

impl VocabIntermidiate {
    pub(crate) fn to_real(&self) -> Vocab {
        // let length = self.b2t.clone().into_values().max().unwrap() + 1;
        // dbg!(length);
        // let mut t2b: Vec<Option<TinyString>> = Vec::with_capacity(length as usize);
        // t2b.fill(const { None });
        // for (k, v) in self.b2t.iter() {
        //     dbg!(v);
        //     t2b[*v as usize] = Some(TinyString::new(&k));
        // }
        // let t2b: Vec<TinyString> = t2b.into_iter().filter_map(|x| x).collect();

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
#[pyclass]
pub struct Vocab {
    pub b2t: BTreeMap<TinyString, u64>,
    pub t2b: Vec<TinyString>,
}

impl Vocab {
    pub fn new(x: Bytes2Token) -> Vocab {
        let length = x.clone().into_values().max().unwrap() + 1;
        dbg!(length);
        let mut t2b: Vec<Option<TinyString>> = vec![None; length as usize];
        // t2b.fill(const { None });
        for (k, v) in x.iter() {
            dbg!(v);
            t2b[*v as usize] = Some(k.clone());
        }
        let t2b: Vec<TinyString> = t2b.into_iter().filter_map(|x| x).collect();

        Vocab { b2t: x, t2b: t2b }
    }

    pub(crate) fn to_intermidiate(&self) -> VocabIntermidiate {
        VocabIntermidiate {
            b2t: self.b2t.iter().map(|(k, v)| (k.to_string(), *v)).collect(),
            t2b: self.t2b.iter().map(|x| x.to_string()).collect(),
        }
    }
}
