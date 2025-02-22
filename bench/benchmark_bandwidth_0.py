from veloxbpe import *
import tiktoken
import time

def bench_veloxbpe(source: str):
    tokenizer = tokenizer_for_vocab("o200k_base")
    tokenizer.py_encode(source) ## warm up

    t0 = time.perf_counter_ns()
    tokenizer.py_encode(source) 
    t1 = time.perf_counter_ns()
    t_delta = t1-t0
    return t_delta


def bench_tiktoken(source: str):
    tokenizer = tiktoken.get_encoding("o200k_base")
    tokenizer.encode(source) ## warm up

    t0 = time.perf_counter_ns()
    tokenizer.encode(source) 
    t1 = time.perf_counter_ns()
    t_delta = t1-t0
    return t_delta



def main(): 
    d="hello"
    with open("./data/sample0.txt", "r") as f: d=f.read()
    size = len(bytes(d.encode("utf-8")))

    tdelta_veloxbpe = bench_veloxbpe(d) / 1e9
    tdelta_tiktoken = bench_tiktoken(d) / 1e9

    n = 10_000_000
    bandwidth_veloxbpe_v = []
    bandwidth_tiktoken_v = []

    for _ in range(n):
        bandwidth_veloxbpe = size / tdelta_veloxbpe
        bandwidth_tiktoken = size / tdelta_tiktoken

        bandwidth_veloxbpe_v.append(bandwidth_veloxbpe)
        bandwidth_tiktoken_v.append(bandwidth_tiktoken)

    mean_veloxbpe = sum(bandwidth_veloxbpe_v) / n
    mean_tiktoken = sum(bandwidth_tiktoken_v) / n

    print(f"bandwidth :: veloxbpe = {round(mean_veloxbpe / 1e6, 2)} MB/s avg over {n} iterations")
    print(f"bandwidth :: tiktoken = {round(mean_tiktoken / 1e6, 2)} MB/s avg over {n} iterations")



if __name__ == "__main__":
    main()
