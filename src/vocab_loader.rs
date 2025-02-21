use std::{collections::BTreeMap, time::Duration};

use crate::{bytepair::BytePair, vocab::Vocab};

pub trait VocabLoader {
    fn load(&self) -> Vocab;
}

pub struct O200kBase {}
impl VocabLoader for O200kBase {
    fn load(&self) -> Vocab {
        let resp = reqwest::blocking::ClientBuilder::new()
            .timeout(Duration::from_secs(1200))
            .build()
            .unwrap()
            .get("https://openaipublic.blob.core.windows.net/encodings/o200k_base.tiktoken")
            .send()
            .unwrap()
            .text()
            .unwrap();
        println!("{}", resp.len());
        BTreeMap::new()
    }
}
