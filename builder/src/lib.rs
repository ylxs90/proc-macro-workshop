use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::Type::Path;
use syn::{
    DataStruct, FieldsNamed, GenericArgument, Ident, Meta, PathArguments, Type, parse_macro_input,
};
use syn::spanned::Spanned;

#[proc_macro_derive(Builder, attributes(builder))]
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
            #name: None
        }
    });

    let flattened_accessors = fields.iter().filter_map(|f| attr_builder_each(f));

    // let accessors = fields.iter().map(|f| {
    //     let name = f.ident.clone().unwrap();
    //     let ty = extract_inner_from(&f.ty, "Option").unwrap_or(&f.ty);
    //     quote! {
    //         pub fn # name( & mut self, #name: # ty) -> & mut Self {
    //         self.# name = Some( # name);
    //         self
    //         }
    //     }
    // });

    let build = fields.iter().map(|f| {
        let name = f.ident.clone().unwrap();
        if extract_inner_from(&f.ty, "Option").is_some() {
            quote! {
            # name: self.# name.clone()
            }
        } else {
            quote! {
            #name: self.#name.clone().ok_or(concat ! (stringify ! (# name), " is not set")) ?
            }
        }
    });
    quote! {
        pub struct #builder_ident {
               #(#optionalized,)*
        }

        impl #builder_ident {
            #(#flattened_accessors)*

            // #(#accessors)*


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

fn extract_inner_from<'a>(ty: &'a Type, outer: &str) -> Option<&'a Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };

    // 取最后一个 path segment（考虑 std::option::Option<T> 这种写法）
    let segment = type_path.path.segments.last()?;
    if segment.ident != outer {
        return None;
    }

    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };

    // Option<T> 只有一个泛型参数
    args.args.iter().find_map(|arg| {
        if let GenericArgument::Type(inner_ty) = arg {
            Some(inner_ty)
        } else {
            None
        }
    })
}

fn attr_builder_each(f: &syn::Field) -> Option<proc_macro2::TokenStream> {
    let name = f.ident.clone().unwrap();
    let tp = extract_inner_from(&f.ty, "Option").unwrap_or(&f.ty);
    if f.attrs.is_empty() {
        return Some(
            quote! {    pub fn #name(&mut self, #name: #tp) -> &mut Self {
                self.#name = Some(#name);
                self
            }},
        );
    }
    for attr in f.attrs.iter() {
        if attr.path().segments.len() == 1 && attr.path().segments[0].ident == "builder" {
            if let Meta::List(ref l) = attr.meta {
                if let Some(inner_tp) = extract_inner_from(&f.ty, "Vec") {
                    let stream = l.tokens.clone();
                    let mut stream = stream.into_iter();
                    if let Some(e) = stream.next() {
                        if e.to_string() != "each" {

                            return Some(syn::Error::new(l.tokens.span(), "expected `builder(each = \"...\")`").into_compile_error());
                        }
                    }

                    assert_eq!(stream.next().unwrap().to_string(), "=");
                    let var_name = stream.next().unwrap();

                    match var_name {
                        proc_macro2::TokenTree::Literal(i) => {
                            let var_name =
                                syn::Ident::new(&i.to_string().trim_matches('"'), i.span());
                            if var_name == name {
                                return Some(quote! {
                                   pub fn #var_name(&mut self, #var_name: #inner_tp) -> &mut Self {
                                        if let Some(ref mut vec) = self.#name {
                                            vec.push(#var_name);
                                        } else {
                                            self.#name = Some(vec![#var_name]);
                                        }
                                        self
                                    }

                                });
                            } else {
                                return Some(quote! {
                                   pub fn #var_name(&mut self, #var_name: #inner_tp) -> &mut Self {
                                        if let Some(ref mut vec) = self.#name {
                                            vec.push(#var_name);
                                        } else {
                                            self.#name = Some(vec![#var_name]);
                                        }
                                        self
                                   }

                                    pub fn #name(&mut self, #name: #tp) -> &mut Self {
                                        self.#name = Some(#name);
                                        self
                                    }

                                });
                            }
                        }
                        _ => {

                            panic!("expected string literal");
                        }
                    }
                } else {
                    panic!("expected fields is Vec");
                }
            } else {
                panic!("expected `builder(each = \"...\")`");
            }
        }
    }
    Some(
        quote! {   pub fn #name(&mut self, #name: #tp) -> &mut Self {
                self.#name = Some(#name);
                self
            }},
    )
}
