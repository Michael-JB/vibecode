use proc_macro::TokenStream;
use syn::{ItemFn, parse_macro_input};

mod attribute;

#[proc_macro_attribute]
pub fn add(_attr: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(annotated_item as ItemFn);

    attribute::impl_add(&ast)
}
