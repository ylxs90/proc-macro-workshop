use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::Item::Fn;
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::visit_mut::{VisitMut, visit_expr_match_mut};
use syn::{Error, ExprMatch, Item, ItemEnum, Pat, parse_macro_input};

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
pub fn check(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as Item);
    let mut m = Matching(None);
    if let Fn(f) = &mut ast {
        m.visit_item_fn_mut(f);
    }
    let mut output = quote! {#ast};
    if let Some(e) = m.0 {
        output.extend(e.to_compile_error());
    }
    output.into()
}

struct Matching(Option<Error>);

impl VisitMut for Matching {
    fn visit_expr_match_mut(&mut self, i: &mut ExprMatch) {
        let mut arms = vec![];
        let mut sort = None;
        i.attrs.retain(|attr| {
            if attr.path().is_ident("sorted") {
                sort = Some(attr.clone());
                false
            } else {
                true
            }
        });
        let len = i.arms.len();
        i.arms.iter().enumerate().for_each(|(i, f)| {
            if let Pat::TupleStruct(p) = f.pat.clone() {
                arms.push((
                    p.path
                        .segments
                        .iter()
                        .map(|p| p.ident.to_string())
                        .reduce(|acc, e| format!("{}::{}", acc, e)),
                    p.path.segments.span(),
                ));
            }

            if let Pat::Slice(p) = f.pat.clone() {
                if p.elems.is_empty() {
                    self.0 = Some(Error::new(
                        f.pat.span(),
                        format!("unsupported by {}", sort.to_token_stream().to_string()),
                    ));
                    return;
                }
            }

            if let Pat::Wild(_) = f.pat.clone()
                && i != len - 1
            {
                self.0 = Some(Error::new(
                    f.pat.span(),
                    "wild arm should at the end of match block",
                ));
                return;
            }
        });
        let mut new = arms.clone();
        new.sort_by(|a, b| a.0.cmp(&b.0));
        for (i, p) in new.iter().enumerate() {
            let p = p.clone();
            let o = arms[i].clone();
            if o.0 != p.0 {
                self.0 = Some(Error::new(
                    p.1.span(),
                    format!(
                        "{} should sort before {}",
                        p.0.unwrap_or_default(),
                        o.0.unwrap_or_default()
                    ),
                ));
                return;
            }
        }

        visit_expr_match_mut(self, i);
    }
}
