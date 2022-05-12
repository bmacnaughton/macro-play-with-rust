//use rustc_hash::FxHashMap;

// necessary for the TokenStream::from_str() implementation
//use std::str::FromStr;

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

const VMETHODS: &str = r#"
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
    println!("{:?}", input);
    //let ast: syn::Item = syn::parse2(input).unwrap();
    //println!("{:?}", ast);
    //let output: proc_macro2::TokenStream = {
    //    // transform input
    //    ()
    //};
    //quote!(1)
    proc_macro::TokenStream::from(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //show_tokens!(this is some stuff);
    }
}
