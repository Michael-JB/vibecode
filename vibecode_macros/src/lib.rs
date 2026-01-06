use proc_macro::TokenStream;
use syn::{ItemFn, parse_macro_input};

mod ai_responder;
mod attribute;
mod openai;

use darling::FromMeta;

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct VibecodeArgs {
    prompt: Option<String>,
}

#[proc_macro_attribute]
pub fn vibecode(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let args: VibecodeArgs = match syn::parse(attribute) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };
    let item_string = item.to_string();

    let ast = parse_macro_input!(item as ItemFn);
    if !ast.block.stmts.is_empty() {
        panic!("The function body must be empty");
    }

    attribute::impl_vibecode(&item_string, args.prompt.as_deref())
}
