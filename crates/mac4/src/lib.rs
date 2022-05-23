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
pub fn make_lookup_by_str_funct(input: TokenStream) -> TokenStream {
    let LookupThingByStr {
        visibility,
        name,
        type_tuple,
        init,
    } = parse_macro_input!(input as LookupThingByStr);

    if type_tuple.elems.len() != 2 {
        panic!("the tuple must have exactly 2 elements");
    }

    //println!("vis: {:?}", visibility);
    //println!("name: {:?}, to_string(): {}", name, name.to_string());
    //println!("type_tuple (n = {}): tuple: {:?}", type_tuple.elems.len(), type_tuple);
    //println!("type_tuple (n = {}): elems: {:?}", type_tuple.elems.len(), type_tuple.elems);
    //println!("type_tuple.elems[0]: {:?}", type_tuple.elems[0]);

    println!("init elems.len(): {:?}\n", init.elems.len());
    println!("init.elems {:?}\n", init.elems);

    let items_len = init.elems.len();
    for expr in &init.elems {
        if let Expr::Tuple(et) = expr {
            let ExprTuple{elems, ..} = et;
            for e in elems.first() {
                if let syn::Expr::Lit(elit) = e {
                    let ExprLit{lit, ..} = elit;
                    if let syn::Lit::Str(lit_str) = lit {
                        let string = lit_str.token().to_string();
                        let s = &string[1..string.len() - 1];
                        println!("    {:?} ({})", s, s.len());
                        println!("    {:?}", lit_str.token());
                    }
                }
            }
        }
    }
    //let mut tuples: Vec<Expr> = init.clone().elems.into_iter().collect();
    //for elem in &init.elems {
    //    println!("  {:?}", elem);
    //}
    //for expr in tuples {
    //    if let ExprTuple {
    //        attrs,
    //        paren_token,
    //        elems,
    //    } = expr;
    //}

    //if let Type::Reference(tr) = &type_tuple.elems[0] {
    //    //println!("type_ref: {:?}", tr.elem);
    //}

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

    let items_name = format!("{}_ITEMS", name);
    let items_name = Ident::new(&items_name, name.span());
    let items_type = &type_tuple;
    let lookup_type = &type_tuple.elems[0];
    let return_type = &type_tuple.elems[1];
    //let items_len = init.elems.len();

    //let buckets = group_items(type_tuple);

    let expanded = quote! {

        #[allow(non_upper_case_globals)]
        const #items_name: [#items_type; #items_len] = #init;

        #visibility fn #name(s: #lookup_type) -> Option<#return_type> {
            for i in 0..#items_name.len() {
                if s == #items_name[i].0 {
                    return Some(#items_name[i].1);
                }
            }
            None
        }
    };

    TokenStream::from(expanded)
}

//*
use rustc_hash::FxHashMap;
fn group_items(items: &[TypeTuple]) -> FxHashMap<usize, Vec<TypeTuple>> {
    let mut buckets: FxHashMap<usize, Vec<TypeTuple>> = FxHashMap::default();

    for item in items {
        println!("type tuple: {:?}", item);
        //let tokens = item.stream().into_iter().collect::<Vec<_>>();
        //match &tokens[0] {
        //    TokenTree::Literal(str_value) => {
        //        let str = str_value.to_string();
        //        // get rid of the quotes
        //        let str = &str[1..str.len() - 1];
        //        let len = str.len();
        //        let vec = buckets.entry(len).or_insert(Vec::<proc_macro2::Group>::new());
        //        vec.push(item);
//
        //    },
        //    _ => panic!("first item must be string literal")
        //}
    }

    buckets
}

// */
