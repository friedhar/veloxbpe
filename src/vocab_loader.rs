use std::{collections::BTreeMap, time::Duration};

use anyhow::Result;

use crate::{bytepair::BytePair, vocab::Vocab};

pub trait VocabFetcher {
    fn load_raw(&self) -> String;
}

pub struct O200kBase {}
impl VocabFetcher for O200kBase {
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

pub struct VocabLoader<T: VocabFetcher> {
    x: T,
}

fn mkdir_if_needed(path: &str) -> Result<()> {
    if !std::path::Path::new(path).exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

pub const VOCAB_CACHE_DIR: &str = ".veloxbpe";

impl<T: VocabFetcher> VocabLoader<T> {
    pub fn load(&self) -> Result<Vocab> {
        mkdir_if_needed(VOCAB_CACHE_DIR)?;
        Ok(BTreeMap::new())
    }
}
