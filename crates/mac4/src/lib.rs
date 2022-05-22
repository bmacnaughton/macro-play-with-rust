//#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input,
    Token,
    Visibility, Ident, ExprArray, TypeTuple,
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

    //println!("vis: {:?}", visibility);
    //println!("name: {:?}, to_string(): {}", name, name.to_string());
    //println!("type_tuple (n = {}): {:?}", type_tuple.elems.len(), type_tuple);
    //println!("type_tuple.elems[0]: {:?}", type_tuple.elems[0]);
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
    let items_len = init.elems.len();

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
