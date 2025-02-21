use std::{collections::BTreeMap, path::Path, time::Duration};

use anyhow::Result;

use crate::{bytepair::BytePair, vocab::Vocab};

pub trait VocabFetcher {
    fn id() -> &'static str;
    fn load_raw(&self) -> Result<String>;
    fn parse(&self, x: String) -> Result<Vocab>;
}

pub struct O200kBase {}
impl VocabFetcher for O200kBase {
    fn load_raw(&self) -> Result<String> {
        let resp = reqwest::blocking::ClientBuilder::new()
            .timeout(Duration::from_secs(1200))
            .build()
            .unwrap()
            .get("https://openaipublic.blob.core.windows.net/encodings/o200k_base.tiktoken")
            .send()?
            .text()?;
        println!("{}", resp.len());
        Ok(resp)
    }

    fn parse(&self, x: String) -> Result<Vocab> {
        Ok(BTreeMap::new())
    }

    #[inline]
    fn id() -> &'static str {
        "o200k_base"
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

fn read_cached_vocab(path: &Path) -> Result<Vocab> {
    let content = std::fs::read_to_string(path)?;
    let vocab: Vocab = bincode::deserialize(content.as_bytes())?;
    Ok(Vocab::new())
}

pub const VOCAB_CACHE_DIR: &str = ".veloxbpe";

impl<T: VocabFetcher> VocabLoader<T> {
    pub fn load(&self) -> Result<Vocab> {
        mkdir_if_needed(VOCAB_CACHE_DIR)?;
        match read_cached_vocab() {
            Ok(x) => x,
            Err(_) => {
                let raw = self.x.load_raw()?;
                let parsed = self.x.parse(&raw);
            }
        }
        Ok(BTreeMap::new())
    }
}
