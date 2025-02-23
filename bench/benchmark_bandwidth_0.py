from veloxbpe import *
import tiktoken
import time
import matplotlib.pyplot as plt
import numpy as np

PLOT = True

def bench_veloxbpe(source: str):
    tokenizer = Tokenizer("")
    tokenizer.encode(source) ## warm up

    t0 = time.perf_counter_ns()
    o = tokenizer.encode(source) 
    t1 = time.perf_counter_ns()
    t_delta = t1-t0
    return t_delta, o


def bench_tiktoken(source: str):
    tokenizer = tiktoken.get_encoding("o200k_base")
    tokenizer.encode(source) ## warm up

    t0 = time.perf_counter_ns()
    o = tokenizer.encode(source) 
    t1 = time.perf_counter_ns()
    t_delta = t1-t0
    return t_delta, o


def setup_plot(x, us, them):
    width = 0.35  # Bar width

    _, ax = plt.subplots(figsize=(8, 5))

    ax.bar([i - width/2 for i in x], us, width, label="veloxbpe", color="dodgerblue")
    ax.bar([i + width/2 for i in x], them, width, label="tiktoken", color="limegreen")

    ax.set_xlabel("Thread count", fontsize=12)
    ax.set_ylabel("Throughput (MB/s)", fontsize=12)
    ax.set_title("Throughput Comparison", fontsize=14)
    ax.set_xticks(x)
    ax.set_xticklabels(x)
    ax.legend()

    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)
    ax.yaxis.grid(True, linestyle="--", alpha=0.6)


def main(): 
    d="hello"
    with open("./data/sample0.txt", "r") as f: d=f.read()
    size = len(bytes(d.encode("utf-8")))

    tdelta_veloxbpe, output_us = bench_veloxbpe(d) 
    tdelta_tiktoken, output_them = bench_tiktoken(d)
    tdelta_veloxbpe, tdelta_tiktoken = tdelta_veloxbpe / 1e9, tdelta_tiktoken / 1e9 ## ns => s
    print(len(output_us))
    print()
    print()
    print()
    print()
    print(len(output_them))
    print(output_us)
    print(output_them)
    # assert output_us == output_them

    n = 100_000_000
    bandwidth_veloxbpe_v = []
    bandwidth_tiktoken_v = []

    for _ in range(n):
        bandwidth_veloxbpe = size / tdelta_veloxbpe
        bandwidth_tiktoken = size / tdelta_tiktoken

        bandwidth_veloxbpe_v.append(bandwidth_veloxbpe)
        bandwidth_tiktoken_v.append(bandwidth_tiktoken)


    bandwidth_tiktoken_v, bandwidth_veloxbpe_v = np.array(bandwidth_tiktoken_v), np.array(bandwidth_veloxbpe_v)

    mean_veloxbpe =  bandwidth_veloxbpe_v.mean()
    mean_tiktoken =bandwidth_tiktoken_v.mean() 

    print("-"*16 + " CPU: 1 " + "-" * 40)
    print(f"bandwidth :: veloxbpe = {round(mean_veloxbpe / 1e6, 2)} MB/s avg over {n / 1e6}M iterations")
    print(f"bandwidth :: tiktoken = {round(mean_tiktoken / 1e6, 2)} MB/s avg over {n / 1e6}M iterations")
    print("-"*64)


    x = [1]
    veloxbpe_throughput = np.array([mean_veloxbpe/1e6])
    tiktoken_throughput = np.array([mean_tiktoken/1e6])

    if PLOT:
        setup_plot(x, us=veloxbpe_throughput, them=tiktoken_throughput)
        plt.show()



if __name__ == "__main__":
    main()
