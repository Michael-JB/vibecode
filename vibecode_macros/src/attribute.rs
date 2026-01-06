use proc_macro::TokenStream;
use syn::ItemFn;

pub fn impl_add(ast: &ItemFn) -> TokenStream {
    let fn_name = &ast.sig.ident;
    format!("fn {fn_name}(a: u64, b: u64) -> u64 {{ a + b }}")
        .parse()
        .expect("Failed to parse add function")
}
