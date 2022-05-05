use proc_macro::{Ident, Literal, TokenStream};
use quote::quote;
use syn;

#[proc_macro]
pub fn single_command(input: TokenStream) -> TokenStream {
    let mut tt = input.into_iter();
    let mut gen;
    if let Ident(id) = tt.next() {
        if id.into() == "print" {
            if let Literal(string) = tt.next() {
                gen = quote! { println!("{}", string); };
            }
        }
    } else {
        gen = quote! { "no comprendo!" };
    }
    gen.into()
}
