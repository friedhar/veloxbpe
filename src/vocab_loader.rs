use std::{
    collections::BTreeMap,
    io::Write,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::Result;

use crate::{
    base64::base64_decode,
    bytepair::BytePair,
    smallstring::{SmartString, TinyString},
    vocab::{Bytes2Token, Vocab},
};

pub trait VocabFetcher {
    fn id() -> &'static str;
    fn load_raw(&self) -> Result<String>;
    fn parse(&self, x: String) -> Result<Bytes2Token>;
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

    fn parse(&self, x: String) -> Result<Bytes2Token> {
        let lines: Vec<&str> = x.split("\n").collect();
        let o: Bytes2Token = x
            .lines()
            .map(|x| {
                let mut parts = x.split(" ");
                let k = parts.next().unwrap();
                let v = parts.next().unwrap();
                let k_parsed = base64_decode(&k).unwrap();
                let k_parsed = String::from_utf8_lossy(&k_parsed).to_string();

                (SmartString::new(&k_parsed), v.parse::<u64>().unwrap())
            })
            .collect();
        Ok(o)
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

fn cache_vocab(vocab: &Bytes2Token, id: &str) -> Result<()> {
    let mut f = std::fs::File::create(
        PathBuf::new()
            .join(VOCAB_CACHE_DIR)
            .join(format!("{id}.veloxbpe")),
    )?;
    f.write_all(bincode::serialize(&vocab)?.as_slice())?;
    Ok(())
}

pub const VOCAB_CACHE_DIR: &str = ".veloxbpe";

impl VocabLoader<O200kBase> {
    pub fn new() -> VocabLoader<O200kBase> {
        VocabLoader { x: O200kBase {} }
    }
}

impl<T: VocabFetcher> VocabLoader<T> {
    pub fn load(&self) -> Result<Vocab> {
        mkdir_if_needed(VOCAB_CACHE_DIR)?;
        Ok(match self.read_cached_vocab() {
            Ok(x) => x,
            Err(e) => {
                dbg!(&e);
                let raw = self.x.load_raw()?;

                let parsed = self.x.parse(raw)?;

                cache_vocab(&parsed, T::id())?;
                Vocab::new(parsed)
            }
        })
    }

    fn read_cached_vocab(&self) -> Result<Vocab> {
        let content = std::fs::read(self.vocab_cache_path())?;
        let vocab: Vocab = bincode::deserialize(content.as_slice())?;
        Ok(vocab)
    }

    fn vocab_cache_path(&self) -> PathBuf {
        PathBuf::new()
            .join(VOCAB_CACHE_DIR)
            .join(format!("{}.veloxbpe", T::id()))
    }
}
