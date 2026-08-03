use proc_macro::TokenStream;
use syn::__private::TokenStream2;
use syn::Item::Fn;
use syn::spanned::Spanned;
use syn::visit_mut::{VisitMut, visit_expr_match_mut};
use syn::{ExprMatch, Item, ItemEnum, Pat, parse_macro_input};

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = TokenStream2::from(args.clone());
    let mut x = input.clone();
    let ast = parse_macro_input!(input as Item);
    match ast {
        Item::Enum(e) => match handle_enum(e) {
            Ok(_) => {}
            Err(e) => {
                x.extend(TokenStream::from(e.to_compile_error()));
            }
        },
        _ => {
            return syn::Error::new(args.span(), "expected enum or match expression")
                .into_compile_error()
                .into();
        }
    }

    x.into()
}

fn handle_enum(ast: ItemEnum) -> Result<(), syn::Error> {
    let mut array = Vec::new();
    ast.variants.into_iter().for_each(|v| {
        array.push((v.ident.to_string(), v.ident));
    });
    let mut new = array.clone();
    new.sort_by(|a, b| a.0.cmp(&b.0));

    for (i, n) in new.iter().enumerate() {
        // println!("{} {n}", &array[i]);
        if (&array[i].0) != &n.0 {
            let or = &array[i].1;
            let ne = &n.1;
            let span = &n.1.span();
            return Err(syn::Error::new(
                *span,
                format!("{} should sort before {}", ne, or),
            ));
        }
    }

    Ok(())
}

#[proc_macro_attribute]
pub fn check(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_temp = input.clone();
    let ast = parse_macro_input!(input as Item);
    let _args = TokenStream2::from(args);
    let mut m = Matching(None);
    if let Fn(mut f) = ast.clone() {
        m.visit_item_fn_mut(&mut f);
        if let Some(e) = m.0 {
            return e.into_compile_error().into();
        } else {
            return input_temp;
        }
    }
    return input_temp.into();
}

struct Matching(Option<syn::Error>);

impl VisitMut for Matching {
    fn visit_expr_match_mut(&mut self, i: &mut ExprMatch) {
        i.arms.iter().for_each(|f| {
            if let Pat::TupleStruct(p) = f.pat.clone() {
                if p.path.segments[0].ident == "Fmt" {
                    self.0 = Some(syn::Error::new(p.span(), "Fmt should sort before Io"));
                }
            }
        });

        // visit_expr_match_mut(self, i);
    }
}
