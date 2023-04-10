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
//! tradeoff for `#[derive]` macros where compile time is of high importance.
//!
//! # Where to begin
//!
//! `myn` works directly with [`TokenStream`], giving you tools to build your own AST without
//! attempting to define a one-size-fits-all strongly typed AST. The [`TokenStreamExt`] extension
//! trait turns the `TokenStream` into a [`TokenIter`].
//!
//! [`TokenIter`]: crate::ty::TokenIter
//! [`TokenStream`]: proc_macro::TokenStream
//! [`TokenStreamExt`]: crate::traits::TokenStreamExt

#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

extern crate proc_macro;

pub mod prelude;
pub mod traits;
pub mod ty;
pub mod utils;
