use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Expr, ItemFn, LitStr, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::ai_responder::Complexity;

mod ai_responder;
mod attribute;
mod function;
mod openai;

#[derive(FromMeta)]
#[darling(derive_syn_parse)]
struct VibecodeArgs {
    prompt: Option<String>,
    #[darling(default = || Complexity::Low)]
    complexity: Complexity,
}

#[proc_macro_attribute]
pub fn vibecode(attribute: TokenStream, item: TokenStream) -> TokenStream {
    _vibecode(attribute.into(), item.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

struct ViberunArgs {
    prompt: LitStr,
    args: Punctuated<Expr, Token![,]>,
}

#[proc_macro]
pub fn viberun(input: TokenStream) -> TokenStream {
    _viberun(input.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn _vibecode(attribute: TokenStream2, item: TokenStream2) -> syn::Result<TokenStream2> {
    let args: VibecodeArgs = syn::parse2(attribute)?;
    let item_string = item.to_string();
    let ast: ItemFn = syn::parse2(item)?;

    if !ast.block.stmts.is_empty() {
        return Err(syn::Error::new_spanned(
            ast.block,
            "The function body must be empty",
        ));
    }

    Ok(
        attribute::populate_function(&args.complexity, &item_string, args.prompt.as_deref())
            .map_err(|e| {
                syn::Error::new_spanned(&ast.sig, format!("Failed to vibecode function: {}", e))
            })?
            .parse()
            .map_err(|e| {
                syn::Error::new_spanned(
                    &ast.sig,
                    format!("Failed to lex vibecoded function: {}", e),
                )
            })?,
    )
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

fn _viberun(input: TokenStream2) -> syn::Result<TokenStream2> {
    let input: ViberunArgs = syn::parse2(input)?;

    let args = &input.args;
    // TODO make complexity configurable
    let closure =
        function::generate_closure(&Complexity::Low, &input.prompt.value()).map_err(|e| {
            syn::Error::new_spanned(&input.prompt, format!("Failed to vibecode closure: {}", e))
        })?;

    Ok(quote! {(#closure)(#args)})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vibecode_parses_empty() {
        // Given
        let attr = quote! {};

        // When
        let args: VibecodeArgs = syn::parse2(attr).unwrap();

        // Then
        assert_eq!(args.prompt.as_deref(), None);
        assert!(matches!(args.complexity, Complexity::Low));
    }

    #[test]
    fn test_vibecode_parses_prompt() {
        // Given
        let attr = quote! { prompt = "Test" };

        // When
        let args: VibecodeArgs = syn::parse2(attr).unwrap();

        // Then
        assert_eq!(args.prompt.as_deref(), Some("Test"));
    }

    #[test]
    fn test_vibecode_parses_complexity() {
        // Given
        let attr = quote! { complexity = "high" };

        // When
        let args: VibecodeArgs = syn::parse2(attr).unwrap();

        // Then
        assert!(matches!(args.complexity, Complexity::High));
    }

    #[test]
    fn test_viberun_parses_prompt() {
        // Given
        let input = quote! { "Test" };

        // When
        let result: ViberunArgs = syn::parse2(input).unwrap();

        // Then
        assert_eq!(result.prompt.value(), "Test");
        assert!(result.args.is_empty());
    }

    #[test]
    fn test_viberun_parses_args() {
        // Given
        let input = quote! { "Test", 42, true, "foo" };

        // When
        let result: ViberunArgs = syn::parse2(input).unwrap();

        // Then
        assert_eq!(result.prompt.value(), "Test");
        assert_eq!(result.args.len(), 3);
    }

    #[test]
    fn test_viberun_empty_input_fails() {
        // Given
        let input = quote! {};

        // When
        let result: Result<ViberunArgs, _> = syn::parse2(input);

        // Then
        assert!(result.is_err());
    }
}
