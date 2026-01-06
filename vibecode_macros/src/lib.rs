use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, ItemFn, LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

mod ai_responder;
mod attribute;
mod function;
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

struct ViberunArgs {
    prompt: LitStr,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for ViberunArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Err(input.error("viberun! requires at least a prompt string"));
        }
        let prompt: LitStr = input.parse()?;
        let _: Option<Token![,]> = input.parse()?;
        let args = Punctuated::parse_terminated(input)?;
        Ok(ViberunArgs { prompt, args })
    }
}

#[proc_macro]
pub fn viberun(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ViberunArgs);

    let args = &input.args;
    let closure = function::impl_viberun(&input.prompt.value());

    let call_closure = quote! {
        (#closure)(#args)
    };

    TokenStream::from(call_closure)
}
