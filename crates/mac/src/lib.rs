use rustc_hash::FxHashMap;

extern crate proc_macro;

use proc_macro::{
    TokenStream,
    TokenTree,
    //Ident,
};
//use proc_macro2:: {
//    Span,
//};
use quote::{quote};

#[proc_macro]
pub fn show_token_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    display_token_stream(input, 0);

    let output: proc_macro::TokenStream = quote! { () }.into();
    //proc_macro::TokenStream::from(input)
    output
}

fn display_token_stream(input: proc_macro::TokenStream, indent: usize) -> () {
    let inputs = input.clone().into_iter().collect::<Vec<_>>();
    //for input in inputs {
    //    println!("{}{:?}", " ".repeat(indent), input);
    //}
    for input in inputs {
        match &input {
            TokenTree::Group(g) => {
                println!("{}group", " ".repeat(indent));
                display_token_stream(g.stream(), indent + 4);
            },
            _ => {
                println!("{}{:?}", " ".repeat(indent), input);
            }
        }
    }
    ()
}

#[proc_macro]
pub fn build_thing(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // use this to build the output
    let mut output = Vec::<TokenStream>::new();

    let inputs = input.into_iter().collect::<Vec<_>>();
    if inputs.len() != 5 {
        panic!("build_thing!() requires 3 comma-separated arguments");
    }

    let function_name: String;
    match &inputs[0] {
        TokenTree::Ident(func_name) => {
            function_name = func_name.to_string();
        },
        _ => panic!("function name is missing")
    }

    match &inputs[1] {
        TokenTree::Punct(p) if p.to_string() == "," => (),
        _ => panic!("missing comma after function name")
    }

    let type_info: TokenStream;
    match &inputs[2] {
        TokenTree::Group(g) => {
            type_info = g.stream();
        },
        _ => panic!("type information expected")
    }

    match &inputs[3] {
        TokenTree::Punct(p) if p.to_string() == "," => (),
        _ => panic!("missing comma after function name")
    }

    let items;
    match &inputs[4] {
        TokenTree::Group(g) => {
            // iterative parsing of items
            items = build_item_list(g.stream());
        },
        _ => panic!("items expected")
    }

    let span = proc_macro2::Span::call_site();

    let len = items.len();

    let items_decl = quote!(
        const TEST: [&str; #len] = [#(#strings),*];
    );

    let func_name = proc_macro2::Ident::new(&function_name, span);
    let func_def = quote!(
        fn #func_name() -> usize {
            1
        }
    );
    output.push(func_def.into());

    // and return the collected streams
    output.into_iter().collect()
}

fn build_item_list(items: TokenStream) -> Vec<proc_macro::Group> {
    let mut item_groups: Vec<proc_macro::Group> = Vec::new();
    #[derive(PartialEq)]
    enum State {
        NeedGroup,
        MaybeComma,
    }

    let mut state = State::NeedGroup;
    for item in items {
        match item {
            TokenTree::Group(g) => {
                if state != State::NeedGroup {
                    panic!("missing item");
                }
                item_groups.push(g);
                state = State::MaybeComma;
            },
            TokenTree::Punct(p) if p.to_string() == "," => {
                if state != State::MaybeComma {
                    panic!("expected comma");
                }
                state = State::NeedGroup;
            },
            _ => {
                panic!("unexpected token");
            }
        }
    }

    item_groups
}
