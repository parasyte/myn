# Build-time comparisons with `syn`

The benchmark tool used is [`hyperfine`](https://github.com/sharkdp/hyperfine) with the following arguments:

- Warmup: `-w 1`
- Prepare full builds: `-p 'cargo clean'`
- Prepare incremental builds: `-p 'touch src/lib.rs'`
- Command debug builds: `'cargo build'`
- Command release builds: `'cargo build --release'`

Crate versions:

- `myn` @ [c86196dc061a2bea10363c9ed3fb7091a70e3984](https://github.com/parasyte/myn/tree/c86196dc061a2bea10363c9ed3fb7091a70e3984)
- `syn` @ [2dc79880cd982ec217c0e0dbcf751a6e78186b43](https://github.com/dtolnay/syn/tree/2dc79880cd982ec217c0e0dbcf751a6e78186b43)


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
- Intel Core i7-7920HQ CPU @ 3.10GHz (16GB LPDDR3)

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


# Build time comparisons with `#[derive]` macro implementation

The benchmark tool used is [`hyperfine`](https://github.com/sharkdp/hyperfine) with the following arguments:

- Warmup: `-w 1`
- Prepare full builds: `-p 'cargo clean'`
- Prepare incremental builds: `-p 'touch onlyargs_derive/src/lib.rs'`
- Command debug builds: `'cargo build --package derive-example'`
- Command release builds: `'cargo build --package derive-example --release'`

This compares build times between an application using a `#[derive]` macro built on `myn` and `syn`-family crates. The incremental builds in this setup do not rebuild the dependencies (`myn`, `syn`, etc.) at all.

Crate version:

- `onlyargs_derive` @ [3f446544f7ffa4987fae725ddf367f24acb29be5](https://github.com/parasyte/onlyargs/tree/3f446544f7ffa4987fae725ddf367f24acb29be5)
    - `myn` @ [c86196dc061a2bea10363c9ed3fb7091a70e3984](https://github.com/parasyte/myn/tree/c86196dc061a2bea10363c9ed3fb7091a70e3984)
- `onlyargs_derive` @ [6abe5cd5474239846b3bc81d87bca6779e342d1e](https://github.com/parasyte/onlyargs/tree/6abe5cd5474239846b3bc81d87bca6779e342d1e)
    - `syn` @ [2.0.12](https://github.com/dtolnay/syn/tree/2.0.12)
    - `quote` @ [1.0.26](https://github.com/dtolnay/quote/tree/1.0.26)
    - `proc-macro2` @ [1.0.54](https://github.com/dtolnay/proc-macro2/tree/1.0.54)

## Mid-range laptop

Benchmark environment:

- macOS 13.2.1
- Intel Core i7-7920HQ CPU @ 3.10GHz (16GB LPDDR3)

### Time (mean ± σ):

| Crate | Full debug            | Full release          | Incremental debug       | Incremental release   |
|-------|-----------------------|-----------------------|-------------------------|-----------------------|
| `myn` | **2.008 s ± 0.032 s** | **1.982 s ± 0.069 s** | **969.0 ms ±  41.5 ms** | **1.258 s ± 0.012 s** |
| `syn` |   6.931 s ± 0.150 s   |   6.935 s ± 0.088 s   |   1.080 s  ± 0.030 s    |   1.486 s ± 0.027 s   |

### Range (min … max):

| Crate | Full debug            | Full release          | Incremental debug        | Incremental release   |
|-------|-----------------------|-----------------------|--------------------------|-----------------------|
| `myn` | **1.945 s … 2.059 s** | **1.884 s … 2.113 s** | **932.5 ms … 1076.6 ms** | **1.243 s … 1.277 s** |
| `syn` |   6.594 s … 7.138 s   |   6.840 s … 7.148 s   |   1.033 s  …  1.117 s    |   1.447 s … 1.529 s   |
