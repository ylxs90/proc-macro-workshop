use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    //let ast =
        if let Data::Enum(ref e) = ast.data {
        // println!("{:#?}", ast);
        e.clone();
    } else {
        println!("{:#?}", ast);
        return quote! { compile_error!("expected enum or match expression"); }.into();
    };

    quote! { /* ... */}.into()
}
