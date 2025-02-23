use pyo3::prelude::*;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

use crate::smallstring::TinyString;
use std::collections::HashMap;

pub type Bytes2Token = FxHashMap<TinyString, u64>;

#[derive(Deserialize, Serialize, Clone)]
pub struct VocabIntermidiate {
    pub b2t: HashMap<String, u64>,
    pub t2b: HashMap<u64, String>,
}

impl VocabIntermidiate {
    pub(crate) fn to_real(&self) -> Vocab {
        let mut t2b_v: Vec<Option<TinyString>> = Vec::with_capacity(self.b2t.len());
        let length: usize = *self.t2b.keys().max().unwrap() as usize + 1;
        dbg!(length);
        dbg!(self.t2b.len());
        for _ in 0..length {
            t2b_v.push(None);
        }

        for (v, k) in self.t2b.iter() {
            if *v as usize >= length {
                panic!()
            }
            t2b_v[*v as usize] = Some(TinyString::new(&k));
        }

        // let prev_len = t2b_v.len();
        // let t2b_v: Vec<TinyString> = t2b_v.into_iter().filter_map(|x| x).collect();
        // assert_eq!(prev_len, t2b_v.len());

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

            max_word_len: t2b_v
                .iter()
                .map(|x| x.unwrap_or(TinyString::new("")).len())
                .max()
                .unwrap(),
            t2b_seq: t2b_v.into_boxed_slice(),
        }
    }
}

#[derive(Clone)]
#[pyclass]
pub struct Vocab {
    pub b2t: FxHashMap<TinyString, u64>,
    pub t2b: HashMap<u64, TinyString>,
    pub t2b_seq: Box<[Option<TinyString>]>,
    pub max_word_len: usize,
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

        Vocab {
            b2t: x,
            t2b,
            t2b_seq: Box::new([]),
            max_word_len: 0,
        }
    }

    pub(crate) fn to_intermidiate(&self) -> VocabIntermidiate {
        VocabIntermidiate {
            b2t: self.b2t.iter().map(|(k, v)| (k.to_string(), *v)).collect(),
            t2b: self.t2b.iter().map(|(k, v)| (*k, v.to_string())).collect(),
        }
    }
}
