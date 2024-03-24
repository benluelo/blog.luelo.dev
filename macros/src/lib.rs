use proc_macro::{Delimiter, Group, Punct, Spacing, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn apply(meta: TokenStream, ts: TokenStream) -> TokenStream {
    let mut it = meta.into_iter();

    let Some(ident @ TokenTree::Ident(_)) = it.next() else {
        panic!()
    };

    assert!(it.next().is_none());

    [
        ident,
        Punct::new('!', Spacing::Alone).into(),
        TokenTree::Group(Group::new(Delimiter::Brace, ts)),
    ]
    .into_iter()
    .collect()
}
