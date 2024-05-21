# SP1 BN254 benchmark

## Run

Make sure you have SP1 toolchain [installed](https://succinctlabs.github.io/sp1/getting-started/install.html).

```
make
```

## Results

Based on a single run to get rough estimates (TODO: do random sampling for more precise numbers).

| Operation | # cycles |
| - | - |
| G1 decoding (uncompressed) | 10,194 |
| G1 encoding | 119,182 |
| G1 addition | 30,034 |
| G1 multiplication | 1,735,257 |
| G2 decoding (uncompressed) | 27,958,488 |
| G2 encoding | 162,243 |
| G2 addition | 104,122 |
| G2 multiplication | 31,221,329 |
| Miller loop | 33,175,157 |
| Final exponentiation | 45,323,747 |
