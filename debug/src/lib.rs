use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{
    DataStruct, Expr, FieldsNamed, GenericArgument, GenericParam, Generics, Lit, Meta,
    PathArguments, Type, TypePath, parse_macro_input, parse_quote,
};

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let ident = &ast.ident;
    let origin_generics = ast.generics.clone();
    let g = add_trait_bounds(origin_generics.clone());
    let mut gg = g.clone();
    let gt = if !g.params.is_empty()
        && let GenericParam::Type(ref t) = g.params[0]
    {
        Some(t.ident.clone())
    } else {
        None
    };
    let (impl_generics, ty_generics, _w) = g.split_for_impl();
    println!("{:?}", 1111);
    println!("{:#?}", g.split_for_impl());
    let fields = if let Struct(DataStruct {
        fields: Named(FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!()
    };

    let mut need_generics = false;
    let each_fields: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|ref field| {
            let ident = &field.ident;
            let ident = ident.clone().unwrap();
            if !need_generics && let Type::Path(p) = &field.ty {
                let contains = generics_contains(p, &gt);
                // println!("{:?}  --> {contains}", field.ident,);
                // if !contains {
                //     println!("{:#?}", p);
                // }
                need_generics |= contains;
            }

            if !need_generics && let Type::Tuple(t) = &field.ty {
                // println!("{:#?}", t);
                if !t.elems.is_empty() {
                    need_generics |= t.elems.iter().any(|t| {
                        if let Type::Path(p) = t {
                            generics_contains(p, &gt)
                        } else {
                            false
                        }
                    })
                }
            }

            println!("{:#?}", &field.ty);

            if let Some(debug) = extract_debug(field) {
                quote! {
                    field(stringify!(#ident), &format_args!(#debug, &self.#ident))
                }
            } else {
                quote! {
                    field(stringify!(#ident), &self.#ident)
                }
            }
        })
        .collect();

    let impl_generics = if need_generics {
        quote! { #impl_generics }
    } else {
        quote! { #ty_generics }
    };
    println!("{:?}", 11);
    gg.make_where_clause()
        .predicates
        .push(parse_quote!(#origin_generics: std::fmt::Debug));

    quote! {

       impl #impl_generics std::fmt::Debug for #ident #gg {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#ident))
                    #(.#each_fields)*
                    .finish()
            }
        }

    }
    .into()
}

fn extract_debug(field: &syn::Field) -> Option<String> {
    for attr in field.attrs.iter() {
        if let Meta::NameValue(nv) = attr.meta.clone()
            && nv.path.segments.len() == 1
            && nv.path.segments[0].ident == "debug"
        {
            return if let Expr::Lit(l) = nv.value
                && let Lit::Str(s) = l.lit
            {
                Some(s.value())
            } else {
                None
            };
        }
    }
    None
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(std::fmt::Debug));
        }
    }
    generics
}

fn generics_contains(p: &TypePath, g: &Option<Ident>) -> bool {
    // println!("{:?} --> {}", p.path.segments[0].ident, p.path.segments.iter().any(|ref a| g == &Some(a.ident.clone())));
    p.path
        .segments
        .iter()
        .filter(|segment| segment.ident != "PhantomData")
        .any(|ref seg| {
            if g == &Some(seg.ident.clone()) {
                true
            } else {
                let x = match seg.arguments.clone() {
                    PathArguments::None => false,
                    PathArguments::Parenthesized(p) => {
                        println!("{:#?}", p);
                        false
                    }
                    PathArguments::AngleBracketed(a) => {
                        return a.args.iter().any(|a| {
                            if let GenericArgument::Type(Type::Path(p1)) = a {
                                let contains = generics_contains(p1, g);
                                // println!("{:?} --> {}", p.path.segments[0].ident, contains);
                                return contains;
                            } else {
                                false
                            }
                        });
                    }
                };
                x
            }
        })
}

