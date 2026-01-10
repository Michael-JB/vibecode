//! Helper macros for the vibecode crate.

#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Expr, ItemFn, LitStr, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::{ai_responder::Complexity, openai::OpenAI};

mod ai_responder;
mod openai;
mod vibecode;

#[derive(FromMeta)]
#[darling(derive_syn_parse)]
struct VibecodeArgs {
    prompt: Option<String>,
    #[darling(default = || Complexity::Low)]
    complexity: Complexity,
}

/// Attribute macro to vibecode a function implementation given its signature.
///
/// The function body must be empty.
///
/// Optional parameters:
///   - prompt: Additional prompt to guide the vibecoding process. Use this to pass any extra
///     information about the function that may not be captured in the signature.
///   - complexity: The complexity of the function to vibecode. Can be "low", "medium", or "high".
///     Defaults to "low". Vibecode will choose an appropriate model based on the complexity.
#[proc_macro_attribute]
pub fn vibecode(attribute: TokenStream, item: TokenStream) -> TokenStream {
    vibecode_inner(attribute.into(), item.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn vibecode_inner(attribute: TokenStream2, item: TokenStream2) -> syn::Result<TokenStream2> {
    let openai = OpenAI::default().map_err(|e| {
        syn::Error::new_spanned(
            &attribute,
            format!("Failed to initialize OpenAI client: {e}"),
        )
    })?;

    let args: VibecodeArgs = syn::parse2(attribute)?;
    let item_string = item.to_string();
    let ast: ItemFn = syn::parse2(item)?;

    if !ast.block.stmts.is_empty() {
        return Err(syn::Error::new_spanned(
            ast.block,
            "The function body must be empty",
        ));
    }

    let populated_function = vibecode::populate_function(
        &openai,
        &args.complexity,
        &item_string,
        args.prompt.as_deref(),
    )
    .map_err(|e| {
        syn::Error::new_spanned(&ast.sig.ident, format!("Failed to vibecode function: {e}"))
    })?;

    Ok(quote! { #populated_function })
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

/// Function macro to vibecode and execute a function inline.
///
/// The first parameter is a prompt string describing the function to vibecode. This is required.
/// Subsequent parameters are variadic arguments to pass to the vibecoded function upon execution.
#[proc_macro]
pub fn viberun(input: TokenStream) -> TokenStream {
    viberun_inner(input.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn viberun_inner(input: TokenStream2) -> syn::Result<TokenStream2> {
    let openai = OpenAI::default().map_err(|e| {
        syn::Error::new_spanned(&input, format!("Failed to initialize OpenAI client: {e}"))
    })?;

    let input: ViberunArgs = syn::parse2(input)?;

    let args = &input.args;
    // TODO make complexity configurable
    let closure = vibecode::generate_closure(&openai, &Complexity::Low, &input.prompt.value())
        .map_err(|e| {
            syn::Error::new_spanned(&input.prompt, format!("Failed to vibecode closure: {e}"))
        })?;

    Ok(quote! { (#closure)(#args) })
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
