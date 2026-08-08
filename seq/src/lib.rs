use proc_macro::TokenStream;
use proc_macro2::TokenTree::Group;
use proc_macro2::{Ident, Literal, TokenTree};
use quote::{ToTokens, quote};
use std::ops::RangeFull;
use std::range::Range;
use syn::parse::{Parse, ParseStream};
use syn::{Block, Error, LitInt, Token, parse_macro_input};

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let seq = parse_macro_input!(input as SeqMacro);

    let var = seq.var;
    let mut codes = vec![];
    seq.range.iter().for_each(|i| {
        seq.block.stmts.iter().for_each(|stmt| {
            let map = stmt
                .to_token_stream()
                .into_iter()
                .map(|t| {
                    match t.clone() {
                        TokenTree::Group(g) => {
                            if g.stream().to_string() == var.to_string() {
                                return g
                                    .to_token_stream()
                                    .into_iter()
                                    .map(|t| {
                                        match t {
                                            TokenTree::Ident(t) => {
                                                return TokenTree::Literal(Literal::i32_suffixed(
                                                    i,
                                                ))
                                                .to_token_stream();
                                            }
                                            _ => {}
                                        }
                                        t.to_token_stream()
                                    })
                                    .collect::<Vec<proc_macro2::TokenStream>>();
                            }
                        }
                        _ => {}
                    }
                    t.to_token_stream()
                })
                .collect::<Vec<proc_macro2::TokenStream>>();
            // println!("{:#?}", map);

            codes.push(quote::quote! { #(#map)* });
        })
    });

    quote! { #(#codes)* }.into()
}

#[derive(Debug)]
struct SeqMacro {
    var: Ident,
    range: Range<i32>,
    block: Block,
}

impl Parse for SeqMacro {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let var: Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let start: i32 = input.parse::<LitInt>()?.base10_parse()?;
        input.parse::<Token![..]>()?;
        let end: i32 = if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            input.parse::<LitInt>()?.base10_parse::<i32>()? + 1
        } else {
            input.parse::<LitInt>()?.base10_parse()?
        };

        let range = Range::from(start..end);

        let block = input.parse::<Block>()?;

        Ok(SeqMacro { var, range, block })
    }
}
