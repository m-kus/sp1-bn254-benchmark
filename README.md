# SP1 BN254 benchmark

Benchmarking original and [patched](https://github.com/m-kus/substrate-bn-sp1) `bn` crate that uses `sys_bigint` precompile for modular multiplication.  

## Run

Make sure you have SP1 toolchain [installed](https://succinctlabs.github.io/sp1/getting-started/install.html).

```
make
```

## Results

Based on a single run to get rough estimates (TODO: do random sampling for more precise numbers).

| Operation | substrate-bn | substrate-bn-sp1 (patched) |
| - | - | - |
| G1 decoding (uncompressed) | 10,194 | 2,022 |
| G1 encoding | 119,182 | 101,621 |
| G1 addition | 30,034 | 5,301 |
| G1 multiplication | 1,735,257 |  402,823 |
| G2 decoding (uncompressed) | 27,958,488 | 7,798,137 |
| G2 encoding | 162,243 | 117,460 |
| G2 addition | 104,122 | 31,819 |
| G2 multiplication | 31,221,329 | 8,644,149 |
| Miller loop | 33,175,157 | 9,449,627 |
| Final exponentiation | 45,323,747 | 14,574,801 |
