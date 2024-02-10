//! Extension traits.
//!
//! The primary trait is [`TokenIterExt`], which provides the parsers.

use crate::ty::{Attribute, TokenIter};
use crate::utils::spanned_error;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Span, TokenStream, TokenTree};

/// An extension trait for [`TokenStream`].
pub trait TokenStreamExt {
    /// Turn this type into a [`TokenIter`].
    fn into_token_iter(self) -> TokenIter;
}

/// An extension trait for [`TokenIter`].
///
/// This trait provides parsers and shorthand methods for common getter patterns.
pub trait TokenIterExt: Iterator<Item = TokenTree> {
    /// Parse the input iterator into a list of attributes.
    ///
    /// # Errors
    ///
    /// Returns a compiler error if parsing fails. The error should be inserted into the
    /// `proc_macro` stream.
    fn parse_attributes(&mut self) -> Result<Vec<Attribute>, TokenStream>;

    /// Parse the input iterator as a type visibility modifier.
    ///
    /// E.g. `pub` or `pub(super)`.
    ///
    /// This parser currently discards the result, since it doesn't have much use in a derive macro.
    ///
    /// # Errors
    ///
    /// Returns a compiler error if parsing fails. The error should be inserted into the
    /// `proc_macro` stream.
    fn parse_visibility(&mut self) -> Result<(), TokenStream>;

    /// Parse the input iterator as a path into a string/span pair.
    ///
    /// E.g. `std::collections::HashMap<i32, String>`.
    ///
    /// Due to current limitations in the [`Span`] API, the returned span only points at the span
    /// for the first path segment. For example, it would be `std` in the path above.
    ///
    /// # Errors
    ///
    /// Returns a compiler error if parsing fails. The error should be inserted into the
    /// `proc_macro` stream.
    fn parse_path(&mut self) -> Result<(String, Span), TokenStream>;

    /// Parse the input as a group, expecting the given delimiter.
    ///
    /// Returns the group's inner [`TokenStream`] as a [`TokenIter`] when successful.
    ///
    /// This method should always consume the next item from the stream, even when an error is
    /// returned.
    ///
    /// # Errors
    ///
    /// Returns a compiler error if parsing fails. The error should be inserted into the
    /// `proc_macro` stream.
    fn expect_group(&mut self, expect: Delimiter) -> Result<TokenIter, TokenStream>;

    /// Parse the input as an identifier, expecting it to match the given string.
    ///
    /// This method should always consume the next item from the stream, even when an error is
    /// returned.
    ///
    /// # Errors
    ///
    /// Returns a compiler error if parsing fails. The error should be inserted into the
    /// `proc_macro` stream.
    fn expect_ident(&mut self, expect: &str) -> Result<(), TokenStream>;

    /// Parse the input as punctuation, expecting it to match the given char.
    ///
    /// This method should always consume the next item from the stream, even when an error is
    /// returned.
    ///
    /// # Errors
    ///
    /// Returns a compiler error if parsing fails. The error should be inserted into the
    /// `proc_macro` stream.
    fn expect_punct(&mut self, expect: char) -> Result<(), TokenStream>;

    /// Try to parse the input as a group.
    ///
    /// This method should not consume the next item from the stream when an error is returned.
    /// An implementation which always consumes will make it difficult for parsers to try
    /// alternative matches.
    ///
    /// # Errors
    ///
    /// Returns a compiler error if parsing fails. The error should be inserted into the
    /// `proc_macro` stream.
    fn try_group(&mut self) -> Result<Group, TokenStream>;

    /// Try to parse the input as an identifier.
    ///
    /// This method should not consume the next item from the stream when an error is returned.
    /// An implementation which always consumes will make it difficult for parsers to try
    /// alternative matches.
    ///
    /// # Errors
    ///
    /// Returns a compiler error if parsing fails. The error should be inserted into the
    /// `proc_macro` stream.
    fn try_ident(&mut self) -> Result<Ident, TokenStream>;

    /// Try to parse the input as a literal.
    ///
    /// This method should not consume the next item from the stream when an error is returned.
    /// An implementation which always consumes will make it difficult for parsers to try
    /// alternative matches.
    ///
    /// # Errors
    ///
    /// Returns a compiler error if parsing fails. The error should be inserted into the
    /// `proc_macro` stream.
    fn try_lit(&mut self) -> Result<Literal, TokenStream>;

    /// Try to parse the input as punctuation.
    ///
    /// This method should not consume the next item from the stream when an error is returned.
    /// An implementation which always consumes will make it difficult for parsers to try
    /// alternative matches.
    ///
    /// # Errors
    ///
    /// Returns a compiler error if parsing fails. The error should be inserted into the
    /// `proc_macro` stream.
    fn try_punct(&mut self) -> Result<Punct, TokenStream>;
}

/// An extension trait for [`TokenTree`].
pub trait TokenTreeExt {
    /// Get a span from the given [`TokenTree`].
    fn as_span(&self) -> Span;
}

/// An extension trait for [`Literal`].
pub trait LiteralExt {
    /// Parse a literal into a char.
    ///
    /// # Errors
    ///
    /// Returns a compiler error if parsing fails. The error should be inserted into the
    /// `proc_macro` stream.
    fn as_char(&self) -> Result<char, TokenStream>;

    /// Parse a literal into a string.
    ///
    /// # Errors
    ///
    /// Returns a compiler error if parsing fails. The error should be inserted into the
    /// `proc_macro` stream.
    fn as_string(&self) -> Result<String, TokenStream>;
}

impl TokenStreamExt for TokenStream {
    fn into_token_iter(self) -> TokenIter {
        self.into_iter().peekable()
    }
}

impl TokenTreeExt for Option<TokenTree> {
    fn as_span(&self) -> Span {
        match self {
            Some(TokenTree::Group(group)) => group.span(),
            Some(TokenTree::Ident(ident)) => ident.span(),
            Some(TokenTree::Punct(punct)) => punct.span(),
            Some(TokenTree::Literal(lit)) => lit.span(),
            None => Span::call_site(),
        }
    }
}

impl LiteralExt for Literal {
    fn as_char(&self) -> Result<char, TokenStream> {
        let string = self.to_string();
        if !string.starts_with('\'') || !string.ends_with('\'') {
            return Err(spanned_error("Expected char literal", self.span()));
        }

        // Strip single quotes.
        string
            .chars()
            .nth(1)
            .ok_or_else(|| spanned_error("Expected char literal", self.span()))
    }

    fn as_string(&self) -> Result<String, TokenStream> {
        let string = self.to_string();
        if !string.starts_with('"') || !string.ends_with('"') {
            return Err(spanned_error("Expected string literal", self.span()));
        }

        // Strip double quotes and escapes.
        Ok(string[1..string.len() - 1]
            .replace(r#"\""#, r#"""#)
            .replace(r"\n", "\n")
            .replace(r"\r", "\r")
            .replace(r"\t", "\t")
            .replace(r"\'", "'")
            .replace(r"\\", r"\"))
    }
}
