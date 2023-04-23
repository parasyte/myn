//! Minimalist Rust syntax parsing for procedural macros.
//!
//! # Rationale
//!
//! You may wonder why this is even a thing, since `syn` already exists, is a well-supported and
//! excellent crate, and it supports the entire gamut of Rust syntax. In short, `syn` hurts compile
//! times and is almost certainly overkill for your use case.
//!
//! Instead, we prefer a "pay for what you use" model. This small surface area affords rapid compile
//! times at the cost of being able to parse the entirety of Rust language syntax. This is right
//! tradeoff for `#[derive]` macros when compile time is of high importance.
//!
//! For more on compile times, see the [benchmarks].
//!
//! # Limitations
//!
//! This is not intended to be an exhaustive list, but serves as a guide to help determine whether
//! `myn` is suitable for your use case.
//!
//! - Only designed to parse a small subset of Rust language syntax. Mostly just `struct` and `enum`
//!   types.
//! - Can only be used in `proc-macro` crates. This rules out using the library in tests.
//!
//! # Where to begin
//!
//! `myn` works directly with [`TokenStream`], giving you tools to build your own AST without
//! attempting to define a one-size-fits-all strongly typed AST. The [`TokenStreamExt`] extension
//! trait turns the `TokenStream` into a [`TokenIter`].
//!
//! The [`onlyargs`] and [`onlyerror`] crates are good examples of how to use the library.
//!
//! [benchmarks]: https://github.com/parasyte/myn/blob/main/benchmarks.md
//! [`onlyargs`]: https://github.com/parasyte/onlyargs
//! [`onlyerror`]: https://github.com/parasyte/onlyerror
//! [`TokenIter`]: crate::ty::TokenIter
//! [`TokenStream`]: proc_macro::TokenStream
//! [`TokenStreamExt`]: crate::traits::TokenStreamExt

#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![cfg_attr(test, allow(clippy::cmp_owned))]

#[cfg(not(test))]
extern crate proc_macro;

pub mod prelude;
pub mod traits;
pub mod ty;
pub mod utils;
