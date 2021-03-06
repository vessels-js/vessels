mod export;
mod kind;
mod object;
mod share;

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse;
use synstructure::decl_derive;

decl_derive!([Kind, attributes(kind)] => kind::derive);

decl_derive!([Share] => share::derive);

#[proc_macro_attribute]
pub fn object(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse(item.clone()).unwrap_or_else(|_| panic!("expected trait"));
    let extension = object::build(attribute.into(), &mut item);
    let mut item = item.into_token_stream();
    item.extend(extension);
    item.into()
}

#[proc_macro_attribute]
pub fn kind(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse(item.clone()).unwrap_or_else(|_| panic!("expected trait implementation"));
    kind::annotate(&mut item);
    item.into_token_stream().into()
}

#[proc_macro]
pub fn export(item: TokenStream) -> TokenStream {
    export::build(item.into()).into()
}
