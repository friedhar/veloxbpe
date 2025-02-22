from veloxbpe import *
import tiktoken
import time

def main():
    # with open("./data/sample0.txt", "r") as f: d=f.read()
    tokenizer1 = tokenizer_for_vocab("")
    tokenizer2 = tiktoken.get_encoding("o200k_base")
    d="hello"

    st = time.perf_counter_ns() 
    print(tokenizer1.py_encode(d))
    took = time.perf_counter_ns() -st

    print("benchmark: ", took/1000)
    st = time.perf_counter_ns() 
    print(tokenizer2.encode(d))
    took = time.perf_counter_ns() -st
    print("benchmark: ", took/1000)

if __name__ == "__main__":
    main()
