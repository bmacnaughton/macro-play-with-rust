//#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{parse_macro_input, ExprArray, Ident, Token, TypeSlice, Visibility};

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
///         pub const func_name: (&str, result_type) = $EXPR;
///     }
struct LookupThingByStr {
    visibility: Visibility,
    name: Ident,
    type_slice: TypeSlice,
    init: ExprArray,
}

impl Parse for LookupThingByStr {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility: Visibility = input.parse()?;
        input.parse::<Token![const]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let type_slice: TypeSlice = input.parse()?;
        input.parse::<Token![=]>()?;
        let init: ExprArray = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(LookupThingByStr {
            visibility,
            name,
            type_slice,
            init,
        })
    }
}

#[proc_macro]
pub fn make_lookup_by_str_funct(input: TokenStream) -> TokenStream {
    let LookupThingByStr {
        visibility,
        name,
        type_slice,
        init,
    } = parse_macro_input!(input as LookupThingByStr);

    println!("vis: {:?}", visibility);
    println!("name: {:?}, to_string(): {}", name, name.to_string());
    println!("type: {:?}", type_slice);
    println!("type_slice: {:?}", type_slice.elem);
    println!("init expr: {:?} init len: {}", init, init.elems.len());


    // Assert that the static type implements Sync. If not, user sees an error
    // message like the following. We span this assertion with the field type's
    // line/column so that the error message appears in the correct place.
    //
    //     error[E0277]: the trait bound `*const (): std::marker::Sync` is not satisfied
    //       --> src/main.rs:10:21
    //        |
    //     10 |     const PTR: *const [()] = [&()];
    //        |                ^^^^^^^^^^^ `*const [()]` cannot be shared between threads safely
    //let _assert_sync = quote_spanned! {ty.span()=>
    //    struct _AssertSync where #ty: std::marker::Sync;
    //};

    // Check for Sized. Not vital to check here, but the error message is less
    // confusing this way than if they get a Sized error in one of our
    // implementation details where it assumes Sized.
    //
    //     error[E0277]: the trait bound `str: std::marker::Sized` is not satisfied
    //       --> src/main.rs:10:19
    //        |
    //     10 |     const A: [str] = [""];
    //        |               ^^^ `str` does not have a constant size known at compile-time
    //let _assert_sized = quote_spanned! {ty.span()=>
    //struct _AssertSized where #ty: std::marker::Sized;
    //};

    let items_name = format!("{}_ITEMS", name);
    let items_name = Ident::new(&items_name, name.span());
    let items_type = type_slice.elem;
    let items_len = init.elems.len();

    let expanded = quote! {

        #[allow(non_upper_case_globals)]
        const #items_name: [#items_type; #items_len] = #init;

        #visibility fn #name(s: &str) -> Option<usize> {
            for i in 0..#items_name.len() {
                if s == #items_name[i] {
                    return Some(i);
                }
            }
            None
        }
    };

    TokenStream::from(expanded)
}
