from veloxbpe import *
import tiktoken
import time
import matplotlib.pyplot as plt
import numpy as np
from typing import List

PLOT = True

def bench_veloxbpe(source: str, threads: int):
    tokenizer = Tokenizer("", threads)
    tokenizer.encode_batch(source) ## warm up

    t0 = time.perf_counter_ns()
    o = tokenizer.encode_batch(source) 
    t1 = time.perf_counter_ns()
    t_delta = t1-t0
    return t_delta, o


def bench_tiktoken(documents: List[str], threads: int):
    tokenizer = tiktoken.get_encoding("o200k_base")
    tokenizer.encode_batch(documents, num_threads=threads) ## warm up

    t0 = time.perf_counter_ns()
    o = tokenizer.encode_batch(documents, num_threads=threads) 
    t1 = time.perf_counter_ns()
    t_delta = t1-t0
    return t_delta, o


def setup_plot(x, us, them):
    width = 0.35  # Bar width

    _, ax = plt.subplots(figsize=(8, 5))
    # x = range(len(us))

    ax.bar([i - width/2 for i in range(len(x))], us, width, label="veloxbpe", color="dodgerblue")
    ax.bar([i + width/2 for i in range(len(x))], them, width, label="tiktoken", color="limegreen")

    ax.set_xlabel("Thread count", fontsize=12)
    ax.set_ylabel("Throughput (MB/s)", fontsize=12)
    ax.set_title("Throughput Comparison", fontsize=14)
    # ax.set_xticks(x)

    x = [0, *x]
    ax.set_xticklabels(x)
    ax.legend()

    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)
    ax.yaxis.grid(True, linestyle="--", alpha=0.6)

def benchmark(documents: List[str], threads: int, size: int):
    n = int(1)
    bandwidth_veloxbpe_v = []
    bandwidth_tiktoken_v = []

    for _ in range(n):
        tdelta_veloxbpe, _ = bench_veloxbpe(documents, threads=threads) 
        tdelta_tiktoken, _ = bench_tiktoken(documents, threads=threads)
        tdelta_veloxbpe, tdelta_tiktoken = tdelta_veloxbpe / 1e9, tdelta_tiktoken / 1e9 ## ns => s
        
        bandwidth_veloxbpe = size / tdelta_veloxbpe
        bandwidth_tiktoken = size / tdelta_tiktoken

        bandwidth_veloxbpe_v.append(bandwidth_veloxbpe)
        bandwidth_tiktoken_v.append(bandwidth_tiktoken)

    bandwidth_tiktoken_v, bandwidth_veloxbpe_v = np.array(bandwidth_tiktoken_v), np.array(bandwidth_veloxbpe_v)

    veloxbpe =  np.median(bandwidth_veloxbpe_v)
    tiktoken = np.median(bandwidth_tiktoken_v) 
    return veloxbpe, tiktoken

def main(): 
    d="hello"
    with open("./data/sample0.txt", "r") as f: d=f.read()
    n = 128
    size = len(bytes(d.encode("utf-8")))
    size = size * n
    documents = [d for _ in range(n)]

    # assert output_us == output_them
    threads = [1,2,4,8,16,32,64]
    us_v = []
    them_v = []

    for threads_i in threads:
        us_i, them_i = benchmark(documents, threads=threads_i, size=size)
        us_v.append(us_i)
        them_v.append(them_i)

    # print("-"*16 + " CPU: 1 " + "-" * 40)
    # print(f"bandwidth :: veloxbpe = {round(mean_veloxbpe / 1e6, 2)} MB/s avg over {n / 1e6}M iterations")
    # print(f"bandwidth :: tiktoken = {round(mean_tiktoken / 1e6, 2)} MB/s avg over {n / 1e6}M iterations")
    # print("-"*64)


    x = threads
    veloxbpe_throughput = np.array(us_v) / 1e6
    tiktoken_throughput = np.array(them_v) / 1e6

    if PLOT:
        setup_plot(x, us=veloxbpe_throughput, them=tiktoken_throughput)
        plt.show()


if __name__ == "__main__":
    main()
