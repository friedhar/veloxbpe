use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    time::Duration,
};

use anyhow::Result;

use crate::{
    base64::base64_decode,
    smallstring::TinyString,
    vocab::{Bytes2Token, Vocab, VocabIntermidiate},
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
        // println!("{}", resp.len());
        Ok(resp)
    }

    fn parse(&self, x: String) -> Result<Bytes2Token> {
        dbg!(x.lines().last());
        let o: Bytes2Token = x
            // .lines()
            .split("\n")
            .filter(|x| x.len() > 2)
            .map(|x| {
                let mut parts = x.split(" ");
                let k = parts.next().unwrap();
                let v = parts.next().unwrap();
                let k_parsed = base64_decode(&k).unwrap();
                let k_parsed = String::from_utf8_lossy(&k_parsed).to_string();

                (TinyString::new(&k_parsed), v.parse::<u64>().unwrap())
            })
            .collect();
        // println!("parselen: {}", o.len());
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

fn cache_vocab(vocab: &VocabIntermidiate, id: &str) -> Result<()> {
    let mut f = std::fs::File::create(
        PathBuf::new()
            .join(VOCAB_CACHE_DIR)
            .join(format!("{id}.veloxbpe")),
    )?;
    f.write_all(bincode::serialize(&vocab)?.as_slice())?;
    println!("cached");
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
            Ok(x) => x.to_real(),
            Err(e) => {
                dbg!(&e);
                let raw = self.x.load_raw()?;

                let parsed = self.x.parse(raw)?;
                let parsed = Vocab::new(parsed);

                cache_vocab(&parsed.to_intermidiate(), T::id())?;
                parsed
            }
        })
    }

    fn read_cached_vocab(&self) -> Result<VocabIntermidiate> {
        let mut file = File::open(self.vocab_cache_path())?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        let vocab = bincode::deserialize(content.as_slice())?;
        Ok(vocab)
    }

    fn vocab_cache_path(&self) -> PathBuf {
        PathBuf::new()
            .join(VOCAB_CACHE_DIR)
            .join(format!("{}.veloxbpe", T::id()))
    }
}
