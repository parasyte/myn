[![Crates.io](https://img.shields.io/crates/v/myn)](https://crates.io/crates/myn "Crates.io version")
[![Documentation](https://img.shields.io/docsrs/myn)](https://docs.rs/myn "Documentation")
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![GitHub actions](https://img.shields.io/github/actions/workflow/status/parasyte/myn/ci.yml?branch=main)](https://github.com/parasyte/myn/actions "CI")
[![GitHub activity](https://img.shields.io/github/last-commit/parasyte/myn)](https://github.com/parasyte/myn/commits "Commit activity")
[![GitHub Sponsors](https://img.shields.io/github/sponsors/parasyte)](https://github.com/sponsors/parasyte "Sponsors")

Minimalist Rust syntax parsing for procedural macros.

You can think of `myn` as a minimalist crate with similarities to [`syn`](https://docs.rs/syn). It provides utilities to help write procedural macros, but does not attempt to replicate the `syn` types or API.

`myn` exists to support a very small subset of the entire Rust language syntax. Just enough to implement `#[derive]` macros on `struct`s and `enum`s, and that's about it. Everything else is currently out of scope.

## Why

- 100% safe Rust 🦀.
- Write `#[derive]` macros with extremely fast compile times. See [benchmarks](./benchmark.md).

## MSRV Policy

The Minimum Supported Rust Version for `myn` will always be made available in the [MSRV.md](./MSRV.md) file on GitHub.
