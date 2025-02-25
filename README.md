# Faster than OpenAI's Tiktoken


![benchmark bar chart](assets/benchmark_0.png)

volexbpe is a low latency high throughput Byte-Pair encoding derived tokenizer providng exceptional performance & streamline interface.

## Built-In Supported Encodings
* o200k_base - used in o3, o1, gpt-4o.
* cl100k_base - used in gpt-4, gpt-3.5 turbo, gpt-3.5, most openai text embedding endpoints.
* r50k_base - majority decreptad.
* gpt-2 - gpt-2, open source


## Build & Install From Source
Make sure you have `uv`, if not
```sh
curl -LsSf https://astral.sh/uv/install.sh | sh
```
And the Rust toolchain installed, if not
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
then

```sh
git clone https://github.com/friedhar/veloxbpe.git
uv run maturin develop
```

## Benchmark 
All benchmarks can be run locally.
After you've built from source, run
```sh
uv run bench/benchmark_bandwidth_0.py
``` 

## TODO - Possible Road Map
* Add support for custom BPE training.

