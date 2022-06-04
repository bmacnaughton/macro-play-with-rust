//#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{
    Expr,
    parse_macro_input,
    Token,
    Visibility, Ident, ExprArray, TypeTuple, ExprTuple, ExprLit,
};

use rustc_hash::FxHashMap;

/// Parses the following syntax, which is based on dtolnay's work mimicking
/// the `lazy_static` crate:
///
/// https://github.com/dtolnay/syn/blob/master/examples/lazy-static/lazy-static/src/lib.rs
///
///
///     make_lookup_by_str_func! {
///         $VISIBILITY const $NAME: $TYPE = $EXPR;
///     }
///
/// For example:
///
///     make_lookup_by_str_func! {
///         pub const func_name: (&str, usize) = [
///             ("bruce", 5),
///             ("macnaughton", 11),
///         ];
///     }
///
/// generates:
///
///     const func_name_ITEMS: [(&str, usize); 2] = [
///         ("bruce", 5),
///         ("macnaughton", 11),
///     ];
///     pub fn func_name(s: &str) -> Option<usize> {
///         for i in 0..func_name_ITEMS.len() {
///             if s == func_name_ITEMS[i].0 {
///                 return Some(func_name_ITEMS[i].1);
///             }
///         }
///         None
///     }
struct LookupThingByStr {
    visibility: Visibility,
    name: Ident,
    type_tuple: TypeTuple,
    init: ExprArray,
}

impl Parse for LookupThingByStr {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility: Visibility = input.parse()?;
        input.parse::<Token![const]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let type_tuple: TypeTuple = input.parse()?;
        input.parse::<Token![=]>()?;
        let init: ExprArray = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(LookupThingByStr {
            visibility,
            name,
            type_tuple,
            init,
        })
    }
}

#[proc_macro]
pub fn make_lookup_by_str_func(input: TokenStream) -> TokenStream {
    let LookupThingByStr {
        visibility,
        name,
        type_tuple,
        init,
    } = parse_macro_input!(input as LookupThingByStr);

    // todo - allow it to be only one element but return the index? or
    // is that a bridge too far?
    if type_tuple.elems.len() != 1 && type_tuple.elems.len() != 2 {
        panic!("the tuple must have either 1 or 2 elements");
    }

    let span = proc_macro2::Span::call_site();
    let mut buckets: FxHashMap<usize, Vec<ExprTuple>> = FxHashMap::default();

    // this is the ugliest part of the macro - it might just be because i don't
    // really understand them well enough yet. but the only way that i've found
    // to get the `str` is to walk through the tuples and do the nested `if let`
    // sequences until it gets to the string. (well, a state-driven match loop
    // would do it too.) but one way or the other, it must descend to the literal
    // str slices to capture each length to put each `ExprTuple` in the right
    // length bucket.
    //
    // there might be a better way by creating a parser, but i haven't gotten
    // there yet.
    for (ix, expr) in init.elems.iter().enumerate() {
        // most common is a tuple, e.g., ("bruce", EnumThing::Bruce)
        if let Expr::Tuple(et) = expr {
            let ExprTuple{elems, ..} = et;

            for e in elems.first() {
                if let syn::Expr::Lit(elit) = e {
                    let ExprLit{lit, ..} = elit;
                    if let syn::Lit::Str(lit_str) = lit {
                        let string = lit_str.token().to_string();
                        // get rid of double quote at start and end
                        let s = &string[1..string.len() - 1];

                        if !buckets.contains_key(&s.len()) {
                            buckets.insert(s.len(), Vec::<ExprTuple>::new());
                        }
                        let tv = buckets.get_mut(&s.len()).unwrap();
                        tv.push(et.clone());

                    } // else error?
                } // else error?
            }
        // but allow simple lookup of a string, e.g., "bruce"
        } else if let Expr::Lit(elit) = expr {
            let ExprLit{lit, ..} = elit;
            if let syn::Lit::Str(lit_str) = lit {
                let string = lit_str.token().to_string();
                let s = &string[1..string.len() - 1];
                let mut elems = syn::punctuated::Punctuated::<syn::Expr, syn::token::Comma>::new();
                let attrs = Vec::<syn::Attribute>::new();

                elems.push(Expr::Lit(syn::ExprLit{attrs, lit: syn::Lit::Str(lit_str.clone())}));

                let attrs = Vec::<syn::Attribute>::new();
                let index = format!("{}", ix);
                let index = syn::Lit::Int(syn::LitInt::new(&index, span));
                elems.push(Expr::Lit(syn::ExprLit{attrs, lit: index}));
                // need to create ExprTuple for this to flow nicely
                let index_tup = ExprTuple{
                    attrs: Vec::<syn::Attribute>::new(),
                    paren_token: type_tuple.paren_token.clone(),
                    elems
                };

                if !buckets.contains_key(&s.len()) {
                    buckets.insert(s.len(), Vec::<ExprTuple>::new());
                }
                let tv = buckets.get_mut(&s.len()).unwrap();
                tv.push(index_tup);

                //println!("index {} tup: {:?}", ix, index_tup);
            }
        } // else error?
    }

    let mut lengths: Vec<&usize> = buckets.keys().collect();
    lengths.sort();

    use quote::ToTokens;


    // why is declarations a Vec<proc_macro::TokenStream> while match_arms is
    // a Vec::<proc_macro2::TokenStream>, you might ask?
    //
    // well, `declarations` are returned as is from this macro. and macros return
    // proc_macro::TokenStreams. but `match_arms` is used in a `quote!` context and
    // `quote!` both operates on and returns proc_macro2::TokenStreams. specifically,
    // `match_arms` is needed for interpolation via `#(#match_arms),*` and that
    // cannot use proc_macro::TokenStream as input. to sum up my current understanding,
    // `quote!` requires proc_macro2::TokenStreams in order to do iterative
    // interpolation.
    //
    // https://docs.rs/quote/latest/quote/macro.quote.html
    let mut declarations = Vec::<TokenStream>::new();
    let mut match_arms = Vec::<proc_macro2::TokenStream>::new();
    // for each length of the str slices in the user's list build a separate
    // const array of `ExprTuple`s
    let type_tup_stream = type_tuple.to_token_stream();

    for len in lengths {
        let item_vec = buckets.get(len).unwrap();
        let item_count = item_vec.len();

        // make a unique name for the items, using the function name as a prefix and
        // the length as a suffix. because the function name will very unlikely meet
        // the uppercase standard for constants, the `#[alloc(non_upper_case_globals)]`
        // is added.
        let items_name_base = format!("{}_ITEMS_{}", name, len);
        let items_name = syn::Ident::new(&items_name_base, span);
        let itok = items_name.to_token_stream();

        let length_decl = quote!(
            #[allow(non_upper_case_globals)]
            const #itok: [(#type_tup_stream); #item_count] = [#(#item_vec),*];
        );

        declarations.push(length_decl.into());

        // also build a match arm
        let arm = quote!(
            #len => {
                for i in 0..#itok.len() {
                    if s == #itok[i].0 {
                        // function must return Option<type_tuple.1>
                        return Some(#itok[i].1);
                    }
                }
                return None;
            }
        );
        match_arms.push(arm);

    }

    // now build the function using the declarations.
    let lookup_type = &type_tuple.elems[0];
    let return_type = &type_tuple.elems[1];

    let mut expanded = declarations;

    expanded.push(quote! {

        #visibility fn #name(s: #lookup_type) -> Option<#return_type> {
            match s.len() {
                #(#match_arms),*
                _ => None
            }
        }
    }.into());

    // Check for Sized. Not vital to check here, but the error message is less
    // confusing this way than if they get a Sized error in one of our
    // implementation details where it assumes Sized.
    //
    //     error[E0277]: the trait bound `str: std::marker::Sized` is not satisfied
    //       --> src/main.rs:10:19
    //        |
    //     10 |     pub const fn_name: (str, Thing) = [("", Thing::One)];
    //        |                         ^^^^^^^^^^ doesn't have a size known at compile-time
    quote_spanned! (
        type_tuple.elems[0].span()=> struct _x where type_tuple.elems[0]: std::marker::Sized
    );

    // Check the first element of the tuple for PartialEq. Not vital to check here,
    // but the error message is less confusing this way than if they get a PartialEq
    // error in one of our implementation details where it uses `==`.
    //
    // error[E0369]: binary operation `==` cannot be applied to type `Thing`
    //   --> src/main.rs:10:19
    //    |
    // 10 |     const A: (&str, Thing) = [("a", Thing::One), ("b", Thing::Two)];
    //    |
    let _assert_partial_eq = quote_spanned! {type_tuple.elems[0].span()=>
        struct _AssertParialEq where #type_tuple.elems[0]: std::cmd::PartialEq;
    };

    // convert the vector of tokenstreams into a single token stream
    let mut single = TokenStream::new();
    single.extend(expanded);
    //println!("{}", single.to_string());
    single
}
