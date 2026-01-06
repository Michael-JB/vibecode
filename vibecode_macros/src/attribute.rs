use proc_macro::TokenStream;

pub fn impl_add(_signature: &str) -> TokenStream {
    "fn macro_add(a: u64, b: u64) -> u64 {{ a + b }}"
        .parse()
        .expect("Failed to parse add function")
}
