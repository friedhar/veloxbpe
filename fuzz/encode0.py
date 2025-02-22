from veloxbpe import *

tokenizer = tokenizer_for_vocab("o200k_base")
print(tokenizer.py_encode("hello this is called from python FFI is magic!"))
