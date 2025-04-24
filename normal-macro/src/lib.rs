use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn model(attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}
