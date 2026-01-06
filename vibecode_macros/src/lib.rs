use proc_macro::TokenStream;
use syn::{ItemFn, parse_macro_input};

mod ai_responder;
mod attribute;
mod openai;

#[proc_macro_attribute]
pub fn vibecode(_attr: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let item_string = annotated_item.to_string();

    let ast = parse_macro_input!(annotated_item as ItemFn);
    if !ast.block.stmts.is_empty() {
        panic!("The function body must be empty");
    }

    attribute::impl_vibecode(&item_string)
}
