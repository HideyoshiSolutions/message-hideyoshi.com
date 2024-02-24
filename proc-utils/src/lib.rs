use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn guard_resource(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    println!("{} defined", input.sig.ident);
    println!("Args received: {}", _attr.to_string());
    return TokenStream::from(quote!(#input));
}
