# Build-time comparisons with `syn`

The benchmark tool used is [`hyperfine`](https://github.com/sharkdp/hyperfine) with the following arguments:

- Warmup: `-w 1`
- Prepare full builds: `-p 'cargo clean'`
- Prepare incremental builds: `-p 'touch src/lib.rs'`
- Command debug builds: `'cargo build'`
- Command release builds: `'cargo build --release'`

Crate versions:

- `myn` @ [c86196dc061a2bea10363c9ed3fb7091a70e3984](https://github.com/parasyte/myn/commit/c86196dc061a2bea10363c9ed3fb7091a70e3984)
- `syn` @ [2dc79880cd982ec217c0e0dbcf751a6e78186b43](https://github.com/dtolnay/syn/commit/2dc79880cd982ec217c0e0dbcf751a6e78186b43)


## High-end desktop

Benchmark environment:

- Windows 11 Home Version 22H2
- AMD Ryzen 9 5900X @ 3.70 GHz (32GB DDR4)

### Time (mean ± σ):

| Crate | Full debug              | Full release            | Incremental debug      | Incremental release     |
|-------|-------------------------|-------------------------|------------------------|-------------------------|
| `myn` | **331.1 ms ±   5.1 ms** | **334.9 ms ±   9.9 ms** | **232.8 ms ± 10.8 ms** | **301.8 ms ±   8.5 ms** |
| `syn` |   2.933 s  ± 0.013 s    |   3.063 s  ± 0.018 s    |   786.5 ms ± 10.3 ms   |   2.388 s  ± 0.022 s    |

### Range (min … max):

| Crate | Full debug              | Full release            | Incremental debug       | Incremental release     |
|-------|-------------------------|-------------------------|-------------------------|-------------------------|
| `myn` | **323.8 ms … 340.2 ms** | **323.3 ms … 348.9 ms** | **220.4 ms … 253.4 ms** | **287.2 ms … 315.9 ms** |
| `syn` |   2.912 s  … 2.952 s    |   3.042 s  … 3.093 s    |   774.7 ms … 805.4 ms   |   2.360 s  … 2.413 s    |


## Mid-range laptop

Benchmark environment:

- macOS 13.2.1
- Intel(R) Core(TM) i7-7920HQ CPU @ 3.10GHz (16GB LPDDR3)

### Time (mean ± σ):

| Crate | Full debug            | Full release          | Incremental debug       | Incremental release     |
|-------|-----------------------|-----------------------|-------------------------|-------------------------|
| `myn` | **1.008 s ± 0.028 s** | **1.049 s ± 0.040 s** | **339.1 ms ±  21.0 ms** | **618.1 ms ±  20.6 ms** |
| `syn` |   5.980 s ± 0.116 s   |   9.824 s ± 0.094 s   |   1.285 s  ± 0.029 s    |   7.083 s  ± 0.192 s    |

### Range (min … max):

| Crate | Full debug            | Full release          | Incremental debug       | Incremental release      |
|-------|-----------------------|-----------------------|-------------------------|--------------------------|
| `myn` | **0.967 s … 1.052 s** | **0.990 s … 1.117 s** | **320.9 ms … 389.1 ms** | **589.0 ms …  668.8 ms** |
| `syn` |   5.805 s … 6.119 s   |   9.593 s … 9.923 s   |   1.239 s  … 1.333 s    |   6.809 s  …  7.389 s    |
