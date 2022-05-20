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
    show_token_stream, build_thing,
};

#[derive(Debug, Copy, Clone)]
enum QueryType {
    Ne = 1,
    Gt,
    Gte,
}

fn main() {
    show_token_stream!(func_name, (&'static str, QueryType), [
        ("$ne", QueryType::Ne),
        ("$gt", QueryType::Gt),
        ("$gte", QueryType::Gte),
    ]);
    build_thing!(my_func, (&'static str, QueryType), [
        ("$ne", QueryType::Ne),
        ("$gt", QueryType::Gt),
        ("$gte", QueryType::Gte),
    ]);

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim_end();
        if line.len() == 0 {
            break;
        }
        println!("executing my_func(\"{}\") {}", line, my_func(line));
        //let stream: TokenStream = TokenStream::from_str(line).unwrap().into();

        //println!("{:?}\n", stream);
        // /println!("{:?}", tuple_parse!("bruce", "wayne", "craig mac", ));
    }
}
