//! Miscellaneous functions.

use crate::traits::{LiteralExt as _, TokenIterExt as _};
use crate::ty::Attribute;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

/// Create a compiler error with the given span.
pub fn spanned_error<S: AsRef<str>>(msg: S, span: Span) -> TokenStream {
    let mut group = Group::new(
        Delimiter::Parenthesis,
        TokenTree::from(Literal::string(msg.as_ref())).into(),
    );
    group.set_span(span);

    TokenStream::from_iter([
        TokenTree::Ident(Ident::new("compile_error", span)),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(group),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ])
}

/// Get a list of lines representing the doc comment.
#[must_use]
pub fn get_doc_comment(attrs: &[Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter_map(|attr| {
            if attr.name.to_string() == "doc" {
                let mut tree = attr.tree.clone();

                match tree.next() {
                    Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => (),
                    _ => return None,
                }

                tree.try_lit().and_then(|lit| lit.as_string()).ok()
            } else {
                None
            }
        })
        .collect()
}
