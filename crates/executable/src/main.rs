use rustc_hash::FxHashMap;

// necessary for the TokenStream::from_str() implementation
use std::str::FromStr;

extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ItemStruct;

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

const WORDS: [&str; 3] = [
    "bruce",
    "heihei",
    "zane",
];

use mac::{
    show_token_stream, sorted_strings, make_function, words_list, item_struct2
};

fn main() {
    //show_tokens!(const x: [&str; 3] = ["bruce", "says", "hi"];);
    let func = sorted_strings!(const WORDS: [&str; 3] = [
        "bruce",
        "heihei",
        "zane",
    ];);

    /*
    sig.func Ident { ident: "double", span: #0 bytes(1602..1608) }
    sig.args [Path(TypePath { qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "usize", span: #0 bytes(1609..1614) }, arguments: None }] } })]
    sig.ret Path(TypePath { qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "usize", span: #0 bytes(1619..1624) }, arguments: None }] } })
    */
    make_function!(fn double(usize) -> usize);

    println!("double(77) is {}", double(77));

    //show_token_stream!(["bruce", "zane"]);
    // TokenStream [Group { delimiter: Bracket, stream: TokenStream [Literal { kind: Str, symbol: "bruce", suffix: None, span: #0 bytes(2159..2166) }, Punct { ch: ',', spacing: Alone, span: #0 bytes(2166..2167) }, Literal { kind: Str, symbol: "zane", suffix: None, span: #0 bytes(2168..2174) }], span: #0 bytes(2158..2175) }]

    //pub fn get_query_type(s: &str) -> Option<QueryType>
    // TokenStream [Ident { sym: pub }, Ident { sym: fn }, Ident { sym: get_query_type }, Group { delimiter: Parenthesis, stream: TokenStream [Ident { sym: s }, Punct { char: ':', spacing: Alone }, Punct { char: '&', spacing: Alone }, Ident { sym: str }] }, Punct { char: '-', spacing: Joint }, Punct { char: '>', spacing: Alone }, Ident { sym: Option }, Punct { char: '<', spacing: Alone }, Ident { sym: QueryType }, Punct { char: '>', spacing: Alone }]

    //words_list("bruce", "bruc", "bru",);
    words_list!("bruce", "bruc", "bru",);

    println!("TEST = {:?}, T2 = {:?}, T2.len() = {}", TEST, T2, get_T2_len());

    // error:
    // /println!("item_struct2 {:?}", item_struct2!( { bruce: u32 } ));

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim_end();
        if line.len() == 0 {
            break;
        }
        let stream: TokenStream = TokenStream::from_str(line).unwrap().into();

        println!("{:?}\n", stream);
        // /println!("{:?}", tuple_parse!("bruce", "wayne", "craig mac", ));
    }
}

fn _main2() {
    let mut buckets = FxHashMap::<usize, Vec<String>>::default();
    let tokens = TokenStream::from_str(_VMETHODS).unwrap();
    let ast: syn::Item = syn::parse2(tokens).unwrap();
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
}

fn _main1() {
    // struct sample
    let s = "struct Point { x : u16 , y : u16 }";

    // create a new token stream from our string
    let tokens = TokenStream::from_str(s).unwrap();


    // build the AST: note the syn::parse2() method rather than the syn::parse() one
    // which is meant for "real" procedural macros
    let ast: ItemStruct = syn::parse2(tokens).unwrap();

    // save our struct type for future use
    let struct_type = ast.ident.to_string();
    assert_eq!(struct_type, "Point");

    // we have 2 fields
    assert_eq!(ast.fields.len(), 2);

    // syn::Fields is implementing the Iterator trait, so we can iterate through the fields
    let mut iter = ast.fields.iter();

    // this is x
    let x_field = iter.next().unwrap();
    assert_eq!(x_field.ident.as_ref().unwrap(), "x");

    // this is y
    let y_field = iter.next().unwrap();
    assert_eq!(y_field.ident.as_ref().unwrap(), "y");

    // now the most tricky part: use the quote!() macro to generate code, aka a new
    // TokenStream

    // first, build our function name: point_summation
    let function_name = format_ident!("{}_summation", struct_type.to_lowercase());

    // and our argument type. If we don't use the format ident macro, the function prototype
    // will be: pub fn point_summation (pt : "Point")
    let argument_type = format_ident!("{}", struct_type);

    // same for x and y
    let x = format_ident!("{}", x_field.ident.as_ref().unwrap());
    let y = format_ident!("{}", y_field.ident.as_ref().unwrap());

    // the quote!() macro is returning a new TokenStream. This TokenStream is returned to
    // the compiler in a "real" procedural macro
    let summation_fn = quote! {
        pub fn #function_name(pt: &#argument_type) -> u16 {
            pt.#x + pt.#y
        }
    };

    // output our function as Rust code
    println!("{}", summation_fn);
}
