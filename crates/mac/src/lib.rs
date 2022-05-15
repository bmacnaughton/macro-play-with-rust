use rustc_hash::FxHashMap;

// necessary for the TokenStream::from_str() implementation
//use std::str::FromStr;

extern crate proc_macro;

use proc_macro2::{
    TokenStream,
};
use quote::{format_ident, quote};
use syn::{
    ItemStruct,
    parenthesized,
    parse_macro_input,
    Ident, Result, Type, Token,
    parse:: {
        Parse,
        ParseStream,
    },
    punctuated:: {
        Punctuated,
    },
    token,
};

const _VALID_METHODS: [&str; 31] = [
    "ACL",
    "BASELINE-CONTROL",
    "CHECKIN",
    "CHECKOUT",
    "CONNECT",
    "COPY",
    "DELETE",
    "GET",
    "HEAD",
    "LABEL",
    "LOCK",
    "MERGE",
    "MKACTIVITY",
    "MKCALENDAR",
    "MKCOL",
    "MKWORKSPACE",
    "MOVE",
    "OPTIONS",
    "ORDERPATCH",
    "PATCH",
    "POST",
    "PROPFIND",
    "PROPPATCH",
    "PUT",
    "REPORT",
    "SEARCH",
    "TRACE",
    "UNCHECKOUT",
    "UNLOCK",
    "UPDATE",
    "VERSION-CONTROL",
];

const _VMETHODS: &str = r#"
const VALID_METHODS: [&str; 31] = [
    "ACL",
    "BASELINE-CONTROL",
    "CHECKIN",
    "CHECKOUT",
    "CONNECT",
    "COPY",
    "DELETE",
    "GET",
    "HEAD",
    "LABEL",
    "LOCK",
    "MERGE",
    "MKACTIVITY",
    "MKCALENDAR",
    "MKCOL",
    "MKWORKSPACE",
    "MOVE",
    "OPTIONS",
    "ORDERPATCH",
    "PATCH",
    "POST",
    "PROPFIND",
    "PROPPATCH",
    "PUT",
    "REPORT",
    "SEARCH",
    "TRACE",
    "UNCHECKOUT",
    "UNLOCK",
    "UPDATE",
    "VERSION-CONTROL",
];
"#;

#[proc_macro]
pub fn show_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let ast: syn::Item = syn::parse2(input).unwrap();
    //let ast: syn::Item = syn::parse_macro_input!(input).unwrap();
    println!("{:?}", ast);
    let xyzzy = format_ident!("{}", "xyzzy");
    let output: proc_macro::TokenStream = {
        // transform input
        quote!(
            fn #xyzzy() -> u8 {
                1u8
            }
            #xyzzy
        ).into()
    };
    //quote!(1)
    output
    //proc_macro::TokenStream::from(input)
}

#[proc_macro]
pub fn show_token_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    println!("{:?}", input);
    proc_macro::TokenStream::from(input)
}

#[proc_macro]
pub fn sorted_strings(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut buckets = FxHashMap::<usize, Vec<String>>::default();
    //let input = proc_macro2::TokenStream::from(input);
    let ast: syn::Item = syn::parse2(input.into()).unwrap();
    println!("{:?}", ast);

    match ast {
        syn::Item::Const(item_const) => {
            println!("{:?}", item_const.ty);
            println!("{:?}", item_const.expr);
            match *item_const.expr {
                syn::Expr::Array(array) => {
                    for elem in array.elems {
                        match elem {
                            syn::Expr::Lit(expr_lit) => {
                                match expr_lit.lit {
                                    syn::Lit::Str(lit_str) => {
                                        let len = lit_str.value().len();
                                        let vec = buckets.entry(len).or_insert(Vec::<String>::new());
                                        vec.push(lit_str.value());
                                    }
                                    _ => ()
                                }
                            }
                            _ => ()
                        }
                    }
                    let mut keys: Vec<usize> = buckets.keys().cloned().collect();
                    keys.sort_unstable();
                    for key in keys {
                        println!("=> {} {:?}", key, buckets.get(&key).unwrap());
                    }
                },
                _ => (),
            }
        },
        _ => (),
    }
    quote!(1).into()
}

/**
 * http://bitboom.github.io/rust-proc-macro
 */

type Function = Ident;
type Arguments = Vec<Type>;
type Return = Type;

struct Signature {
    function: Function,
    arguments: Arguments,
    return_t: Return,
}

struct Syntax {
    _fn_token: Token!(fn),
    ident: Function,
    _paren_token: token::Paren,
    paren_fields: Punctuated<Type, Token![,]>,
    _rarrow_token: Token!(->),
    return_t: Return,
}

impl Parse for Signature {
    fn parse(stream: ParseStream) -> Result<Self> {
        if stream.is_empty() {
            panic!("Write full function signature.");
        }

        let content;
        let syntax = Syntax {
            _fn_token: stream.parse().unwrap(),
            ident: stream.parse().unwrap(),
            _paren_token: parenthesized!(content in stream),
            paren_fields: content.parse_terminated(Type::parse).unwrap(),
            _rarrow_token: stream.parse().unwrap(),
            return_t: stream.parse().unwrap(),
        };

        Ok(Signature {
            function: syntax.ident,
            arguments: syntax.paren_fields.into_iter().collect(),
            return_t: syntax.return_t,
        })
    }
}

#[proc_macro]
pub fn make_function(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let signature = syn::parse_macro_input!(input as Signature);
    let function = signature.function;
    let arguments = signature.arguments;
    let return_t = signature.return_t;

    if let 1 = arguments.len()  {
        let arg = &arguments[0];
        let tokens = quote!{
            fn #function(arg: #arg) -> #return_t {
                let ret: #return_t = arg * 2;
                println!("input {} * 2 = {}", arg, ret);
                ret
            }
        };
        tokens.into()
    } else {
        panic!("Invalid input");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //show_tokens!(this is some stuff);
    }
}
