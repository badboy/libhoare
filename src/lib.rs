// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// See ../readme.md for an overview.

#[macro_use]
extern crate serde_derive;
extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::fold::{self, Fold};
use syn::{parse_quote, Block, Expr, ItemFn, Stmt};

#[derive(Debug, Deserialize)]
struct Args {
    precond: Option<String>,
    postcond: Option<String>,
    invariant: Option<String>,
}

impl Fold for Args {
    fn fold_item_fn(&mut self, i: ItemFn) -> ItemFn {
        let mut func = fold::fold_item_fn(self, i);
        let old_block = func.block;
        let precond : Block = if let Some(precond) = self.precond.take() {
            let precond_expr = syn::parse_str::<Expr>(&precond).unwrap();
            parse_quote!({ assert!(#precond_expr, concat!("precondition: ", #precond)); })
        } else { parse_quote!({}) };

        let postcond : Block = if let Some(postcond) = self.postcond.take() {
            let postcond_expr = syn::parse_str::<Expr>(&postcond).unwrap();
            parse_quote!({ assert!(#postcond_expr, concat!("postcondition: ", #postcond)); })
        } else { parse_quote!({}) };

        let (invariant_pre, invariant_post) : (Block, Block) = if let Some(invariant) = self.invariant.take() {
            let invariant_expr = syn::parse_str::<Expr>(&invariant).unwrap();
            (
                parse_quote!({ assert!(#invariant_expr, concat!("Invariant entering: ", #invariant)); }),
                parse_quote!({ assert!(#invariant_expr, concat!("Invariant exiting: ", #invariant)); })
            )
        } else {
            (parse_quote!({}), parse_quote!({}))
        };

        let new_block = parse_quote!({
            #precond;
            #invariant_pre;
            let result = (||{ #old_block })();
            #invariant_post;
            #postcond;
            result
        });
        func.block = new_block;
        func
    }

    fn fold_expr(&mut self, e: Expr) -> Expr {
        eprintln!("jer - self: {:?}", self);
        if let Some(precond) = self.precond.take() {
            eprintln!("jer - precond is true. merging.");
            let rest = fold::fold_expr(self, e);
            parse_quote!({
                assert!(#precond);
                #rest
            })
        } else {
            fold::fold_expr(self, e)
        }
    }

    fn fold_stmt(&mut self, s: Stmt) -> Stmt {
        s
    }
}

#[proc_macro_attribute]
pub fn hoare(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ItemFn);

    // Parse the list of variables the user wanted to print.
    let args = args.to_string().replace(",", "\n");
    let mut args : Args = toml::from_str(&args).unwrap();

    let output = args.fold_item_fn(input);

    TokenStream::from(quote!(#output))
}
