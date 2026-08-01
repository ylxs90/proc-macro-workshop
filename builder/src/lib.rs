use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::Type::Path;
use syn::{DataStruct, FieldsNamed, Ident, parse_macro_input};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    // println!("{:#?}", derive_input);
    let ident = ast.ident;
    let builder_name = format!("{}Builder", ident);
    let builder_ident = Ident::new(&builder_name, ident.span());

    let fields = if let Struct(DataStruct {
        fields: Named(FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!()
    };

    let optionalized = fields.iter().map(|f| {
        let name = f.ident.clone();
        let ty = &f.ty;
        if let Path(ty) = &f.ty.clone()
            && ty.path.segments.first().unwrap().ident.to_string() == "Option"
        {
            quote! {
                #name: #ty
            }
        } else {
            quote! {
                #name: std::option::Option<#ty>
            }
        }
    });

    let init = fields.iter().map(|f| {
        let name = f.ident.clone().unwrap();
        quote! {
        # name: None
        }
    });

    let accessors = fields.iter().map(|f| {
        let name = f.ident.clone().unwrap();
        if let Path(ty) = &f.ty.clone()
            && ty.path.segments.first().unwrap().ident.to_string() == "Option"
        {
            eprintln!("{} {:?}", name, ty.path.segments.last());
            quote! {
            pub fn #name( &mut self, #name: #ty) -> & mut Self {
                self.# name = #name;
                self
            }
            }
        } else {
            let ty = &f.ty;
            quote! {
            pub fn # name( & mut self, #name: # ty) -> & mut Self {
            self.# name = Some( # name);
            self
            }
            }
        }
    });

    let build = fields.iter().map(|f| {
        let name = f.ident.clone().unwrap();
        if let Path(ty) = &f.ty.clone()
            && ty.path.segments.first().unwrap().ident.to_string() == "Option"
        {
            quote! {
            # name: self.# name.clone()
            }
        } else {
            quote! {
            # name: self.# name.clone().ok_or(concat ! (stringify ! (# name), " is not set")) ?
            }
        }
    });

    quote! {
        pub struct #builder_ident {
               #(#optionalized,)*
        }

        impl #builder_ident {
            #(#accessors)*

            pub fn build(&mut self) -> Result<#ident, Box<dyn std::error::Error>> {
                Ok(#ident {
                    #(#build,)*
                })
            }
        }

        impl #ident {

            pub fn builder() -> #builder_ident {
                #builder_ident {
                     #(#init,)*
                }
            }


        }
    }
    .into()
}
