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
            let ident = group.try_ident()?;

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
                TokenTree::Punct(punct)
                    if [',', ';'].contains(&punct.as_char()) && nesting == 0 =>
                {
                    break
                }
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
                TokenTree::Group(group) => {
                    span.get_or_insert(group.span());
                    let mut stream = group.stream().into_token_iter();

                    match group.delimiter() {
                        Delimiter::Parenthesis => {
                            // Tuples are comma-separated paths.
                            path.push('(');
                            while stream.peek().is_some() {
                                let (inner, _span) = stream.parse_path()?;
                                path.push_str(&inner);
                                if stream.peek().is_some() {
                                    stream.expect_punct(',')?;
                                    path.push_str(", ");
                                }
                            }
                            if path.ends_with(' ') {
                                path.pop();
                            }
                            path.push(')');
                        }
                        Delimiter::Bracket => {
                            // Arrays are in `[path; size]` form.
                            path.push('[');
                            let (inner, _span) = stream.parse_path()?;
                            path.push_str(&inner);
                            stream.expect_punct(';')?;
                            path.push_str("; ");
                            path.push_str(&stream.try_lit()?.to_string());
                            path.push(']');
                        }
                        _ => return Err(spanned_error("Unexpected token", group.span())),
                    }
                }
                TokenTree::Literal(_) => {
                    return Err(spanned_error("Unexpected token", self.next().as_span()))
                }
            }

            self.next();
        }

        let span =
            span.ok_or_else(|| spanned_error("Unexpected end of stream", Span::call_site()))?;

        Ok((path, span))
    }

    fn expect_group(&mut self, expect: Delimiter) -> Result<TokenIter, TokenStream> {
        self.try_group()
            .and_then(|group| {
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
            .map_err(|err| {
                self.next();
                err
            })
    }

    fn expect_ident(&mut self, expect: &str) -> Result<(), TokenStream> {
        self.try_ident()
            .and_then(|ident| {
                if ident.to_string() == expect {
                    Ok(())
                } else {
                    Err(spanned_error(format!("Expected `{expect}`"), ident.span()))
                }
            })
            .map_err(|err| {
                self.next();
                err
            })
    }

    fn expect_punct(&mut self, expect: char) -> Result<(), TokenStream> {
        self.try_punct()
            .and_then(|punct| {
                if punct.as_char() == expect {
                    Ok(())
                } else {
                    Err(spanned_error(format!("Expected `{expect}`"), punct.span()))
                }
            })
            .map_err(|err| {
                self.next();
                err
            })
    }

    fn try_group(&mut self) -> Result<Group, TokenStream> {
        match self.next_if(|token| matches!(token, TokenTree::Group(_))) {
            Some(TokenTree::Group(group)) => Ok(group),
            tree => Err(spanned_error("Expected group", tree.as_span())),
        }
    }

    fn try_ident(&mut self) -> Result<Ident, TokenStream> {
        match self.next_if(|token| matches!(token, TokenTree::Ident(_))) {
            Some(TokenTree::Ident(ident)) => Ok(ident),
            tree => Err(spanned_error("Expected identifier", tree.as_span())),
        }
    }

    fn try_lit(&mut self) -> Result<Literal, TokenStream> {
        match self.next_if(|token| matches!(token, TokenTree::Literal(_))) {
            Some(TokenTree::Literal(lit)) => Ok(lit),
            tree => Err(spanned_error("Expected literal", tree.as_span())),
        }
    }

    fn try_punct(&mut self) -> Result<Punct, TokenStream> {
        match self.next_if(|token| matches!(token, TokenTree::Punct(_))) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_tokeniter_parse_path() {
        let mut input = TokenStream::from_str("foo::bar").unwrap().into_token_iter();
        assert_eq!(input.parse_path().unwrap().0, "foo::bar");
        assert!(input.next().is_none());

        let mut input = TokenStream::from_str("foo::bar<baz>")
            .unwrap()
            .into_token_iter();
        assert_eq!(input.parse_path().unwrap().0, "foo::bar<baz>");
        assert!(input.next().is_none());

        let mut input = TokenStream::from_str("foo::bar<()>")
            .unwrap()
            .into_token_iter();
        assert_eq!(input.parse_path().unwrap().0, "foo::bar<()>");
        assert!(input.next().is_none());

        let mut input = TokenStream::from_str("foo::bar<(foo, bar::baz<T>)>")
            .unwrap()
            .into_token_iter();
        assert_eq!(
            input.parse_path().unwrap().0,
            "foo::bar<(foo, bar::baz<T>)>"
        );
        assert!(input.next().is_none());

        let mut input = TokenStream::from_str("foo<(bar, [i32; 4])>")
            .unwrap()
            .into_token_iter();
        assert_eq!(input.parse_path().unwrap().0, "foo<(bar, [i32; 4])>");
        assert!(input.next().is_none());

        let mut input = TokenStream::from_str("(u8, )").unwrap().into_token_iter();
        assert_eq!(input.parse_path().unwrap().0, "(u8,)");
        assert!(input.next().is_none());
    }

    #[test]
    fn test_tokeniter_expect_group() {
        let mut input = TokenStream::from_str("{ foo }").unwrap().into_token_iter();
        assert!(input.expect_group(Delimiter::Brace).is_ok());
        assert!(input.next().is_none());

        let mut input = TokenStream::from_str("( foo )").unwrap().into_token_iter();
        assert!(input.expect_group(Delimiter::Brace).is_err());
        assert!(input.next().is_none());

        let mut input = TokenStream::from_str("foo").unwrap().into_token_iter();
        assert!(input.expect_group(Delimiter::Brace).is_err());
        assert!(input.next().is_none());
    }

    #[test]
    fn test_tokeniter_expect_ident() {
        let mut input = TokenStream::from_str("foo").unwrap().into_token_iter();
        assert!(input.expect_ident("foo").is_ok());
        assert!(input.next().is_none());

        let mut input = TokenStream::from_str("bar").unwrap().into_token_iter();
        assert!(input.expect_ident("foo").is_err());
        assert!(input.next().is_none());

        let mut input = TokenStream::from_str("!").unwrap().into_token_iter();
        assert!(input.expect_ident("foo").is_err());
        assert!(input.next().is_none());
    }

    #[test]
    fn test_tokeniter_expect_punct() {
        let mut input = TokenStream::from_str("!").unwrap().into_token_iter();
        assert!(input.expect_punct('!').is_ok());
        assert!(input.next().is_none());

        let mut input = TokenStream::from_str("#").unwrap().into_token_iter();
        assert!(input.expect_punct('!').is_err());
        assert!(input.next().is_none());

        let mut input = TokenStream::from_str("foo").unwrap().into_token_iter();
        assert!(input.expect_punct('!').is_err());
        assert!(input.next().is_none());
    }

    #[test]
    fn test_tokeniter_try_group() {
        let mut input = TokenStream::from_str("{ foo }").unwrap().into_token_iter();
        let expected = Ident::new("foo", Span::call_site());

        let group = input.try_group().unwrap();
        // TODO: This can be replaced with `let else` after MSRV 1.65
        #[allow(clippy::single_match_else)]
        let tree = match group.stream().into_iter().next() {
            Some(TokenTree::Ident(ident)) => ident,
            _ => panic!(),
        };
        assert_eq!(tree, expected);
    }

    #[test]
    fn test_tokeniter_try_group_peek() {
        let mut input = TokenStream::from_str("!").unwrap().into_token_iter();

        assert!(input.try_group().is_err());
        assert!(input.next().is_some());
        assert!(input.next().is_none());
    }

    #[test]
    fn test_tokeniter_try_ident() {
        let mut input = TokenStream::from_str("foo").unwrap().into_token_iter();
        let expected = Ident::new("foo", Span::call_site());

        assert_eq!(input.try_ident().unwrap(), expected);
    }

    #[test]
    fn test_tokeniter_try_ident_peek() {
        let mut input = TokenStream::from_str("!").unwrap().into_token_iter();

        assert!(input.try_ident().is_err());
        assert!(input.next().is_some());
        assert!(input.next().is_none());
    }

    #[test]
    fn test_tokeniter_try_lit() {
        let mut input = TokenStream::from_str("'!'").unwrap().into_token_iter();
        let expected = Literal::character('!').to_string();

        assert_eq!(input.try_lit().unwrap().to_string(), expected);
    }

    #[test]
    fn test_tokeniter_try_lit_peek() {
        let mut input = TokenStream::from_str("!").unwrap().into_token_iter();

        assert!(input.try_lit().is_err());
        assert!(input.next().is_some());
        assert!(input.next().is_none());
    }

    #[test]
    fn test_tokeniter_try_punct() {
        let mut input = TokenStream::from_str("!").unwrap().into_token_iter();
        let expected = '!';

        assert_eq!(input.try_punct().unwrap().as_char(), expected);
    }

    #[test]
    fn test_tokeniter_try_punct_peek() {
        let mut input = TokenStream::from_str("foo").unwrap().into_token_iter();

        assert!(input.try_punct().is_err());
        assert!(input.next().is_some());
        assert!(input.next().is_none());
    }
}
