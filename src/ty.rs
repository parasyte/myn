//! High-level types from the parser.

use crate::traits::{TokenIterExt, TokenStreamExt as _, TokenTreeExt as _};
use crate::utils::spanned_error;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::iter::Peekable;

/// A type alias for the primary [`TokenTree`] iterator.
///
/// It is [`Peekable`] to allow look-ahead of single items.
pub type TokenIter = Peekable<proc_macro::token_stream::IntoIter>;

/// A type representing `#[attributes]`.
pub struct Attribute {
    /// The attribute name.
    ///
    /// This would be `hello` for `#[hello]`.
    pub name: Ident,

    /// The inner [`TokenTree`] iterator.
    pub tree: TokenIter,
}

impl TokenIterExt for TokenIter {
    fn parse_attributes(&mut self) -> Result<Vec<Attribute>, TokenStream> {
        let mut attrs = vec![];

        loop {
            match self.peek() {
                Some(TokenTree::Punct(punct)) if punct.as_char() == '#' => self.next(),
                _ => break,
            };

            let mut group = self.expect_group(Delimiter::Bracket)?;
            let ident = group.as_ident()?;

            attrs.push(Attribute {
                name: ident,
                tree: group.collect::<TokenStream>().into_token_iter(),
            });
        }

        Ok(attrs)
    }

    fn parse_visibility(&mut self) -> Result<(), TokenStream> {
        match self.peek() {
            Some(TokenTree::Ident(ident)) if ident.to_string() == "pub" => self.next(),
            _ => return Ok(()),
        };

        match self.peek() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
                self.next();
            }
            _ => return Ok(()),
        }

        Ok(())
    }

    fn parse_path(&mut self) -> Result<(String, Span), TokenStream> {
        let mut path = String::new();
        let mut span = None;
        let mut nesting = 0;

        while let Some(tree) = self.peek() {
            match tree {
                TokenTree::Punct(punct) if punct.as_char() == ',' && nesting == 0 => break,
                TokenTree::Punct(punct) => {
                    let ch = punct.as_char();

                    // Handle nesting with `<...>`
                    if ch == '<' {
                        nesting += 1;
                    } else if ch == '>' && punct.spacing() == Spacing::Joint {
                        nesting -= 1;
                    }

                    span.get_or_insert_with(|| punct.span());
                    path.push(ch);
                }
                TokenTree::Ident(ident) => {
                    span.get_or_insert_with(|| ident.span());
                    path.push_str(&ident.to_string());
                }
                _ => return Err(spanned_error("Unexpected token", self.next().as_span())),
            }

            self.next();
        }

        let span =
            span.ok_or_else(|| spanned_error("Unexpected end of stream", Span::call_site()))?;

        Ok((path, span))
    }

    fn expect_group(&mut self, expect: Delimiter) -> Result<TokenIter, TokenStream> {
        self.as_group().and_then(|group| {
            let delim = group.delimiter();
            if delim == expect {
                Ok(group.stream().into_token_iter())
            } else {
                let expect = match expect {
                    Delimiter::Brace => "{",
                    Delimiter::Bracket => "[",
                    Delimiter::None => "delimiter",
                    Delimiter::Parenthesis => "(",
                };

                Err(spanned_error(format!("Expected `{expect}`"), group.span()))
            }
        })
    }

    fn expect_ident(&mut self, expect: &str) -> Result<(), TokenStream> {
        self.as_ident().and_then(|ident| {
            if ident.to_string() == expect {
                Ok(())
            } else {
                Err(spanned_error(format!("Expected `{expect}`"), ident.span()))
            }
        })
    }

    fn expect_punct(&mut self, expect: char) -> Result<(), TokenStream> {
        self.as_punct().and_then(|punct| {
            if punct.as_char() == expect {
                Ok(())
            } else {
                Err(spanned_error(format!("Expected `{expect}`"), punct.span()))
            }
        })
    }

    fn as_group(&mut self) -> Result<Group, TokenStream> {
        match self.next() {
            Some(TokenTree::Group(group)) => Ok(group),
            tree => Err(spanned_error("Expected group", tree.as_span())),
        }
    }

    fn as_ident(&mut self) -> Result<Ident, TokenStream> {
        match self.next() {
            Some(TokenTree::Ident(ident)) => Ok(ident),
            tree => Err(spanned_error("Expected identifier", tree.as_span())),
        }
    }

    fn as_lit(&mut self) -> Result<Literal, TokenStream> {
        match self.next() {
            Some(TokenTree::Literal(lit)) => Ok(lit),
            tree => Err(spanned_error("Expected literal", tree.as_span())),
        }
    }

    fn as_punct(&mut self) -> Result<Punct, TokenStream> {
        match self.next() {
            Some(TokenTree::Punct(punct)) => Ok(punct),
            tree => Err(spanned_error("Expected punctuation", tree.as_span())),
        }
    }
}

impl std::fmt::Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("Attribute")
            .field("name", &self.name)
            .field("tree", &"TokenIter {...}")
            .finish()
    }
}
