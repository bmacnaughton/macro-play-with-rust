use rustc_hash::FxHashMap;

// https://stackoverflow.com/questions/61423236/converting-type-inside-quote-gives-trait-errors
extern crate proc_macro2;
use proc_macro2::{
    TokenStream,
    TokenTree,
};
use quote::{quote};

#[proc_macro]
pub fn show_token_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    display_token_stream(input.into(), 0);

    let output: proc_macro::TokenStream = quote! { () }.into();

    output
}

fn display_token_stream(input: TokenStream, indent: usize) -> () {
    let inputs = input.clone().into_iter().collect::<Vec<_>>();
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

///
/// build const arrays of same-length string slices
///
/// build_thing(function_name, tuple_type, [tuple_item, ...]);
///
/// function_name - create a function named function_name and use this as a prefix
/// for the array names.
///
/// tuple_type - defines the pair, e.g., (&'static str, QueryType), for the next argument
///
/// tuple_item - items of tuple_type.
///
/// it creates one constant array of tuple_type for every length of the &'static str slices
/// and creates a function name function_name that takes a &str argument and returns the
/// tuple_item.1 value as a u32. if nothing was matched it returns a 0.
///
/// making the return value an Option comes next but that requires capturing the type of
/// tuple_type.1
///
#[proc_macro]
pub fn build_thing(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // use this to build the output
    let mut output = Vec::<TokenStream>::new();
    let input: TokenStream = input.into();

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

    let mut buckets = FxHashMap::<usize, Vec<proc_macro2::Group>>::default();
    let items;
    let mut lengths: Vec<&usize>;
    match &inputs[4] {
        TokenTree::Group(g) => {
            // parsing the items in the group
            items = build_item_list(g.stream());
            group_items(&mut buckets, items.clone());
            lengths = buckets.keys().collect();
            lengths.sort_unstable();
        },
        _ => panic!("items expected")
    }

    let span = proc_macro2::Span::call_site();

    let mut match_arms = Vec::<TokenStream>::new();
    // for each length of the str slices in the user's list build a separate
    // const.
    for len in lengths {
        let item_vec = buckets.get(len).unwrap();
        let item_count = item_vec.len();
        // get the group as a vec of token streams
        let items_stream: Vec<TokenStream> = item_vec.into_iter().map(|g| g.stream().into()).collect();

        // make a unique name for the items, using the function name as a prefix and
        // the length as a suffix.
        let items_name_base = format!("{}_ITEMS_{}", function_name, len);
        let items_name = proc_macro2::Ident::new(&items_name_base, span);

        // could use group.delimiter to reproduce user's grouping...
        let items_decl = quote!(
            const #items_name: [(#type_info); #item_count] = [#((#items_stream)),*];
        );
        output.push(items_decl.into());

        // also build a match arm
        let arm = quote!(
            #len => {
                for i in 0..#items_name.len() {
                    if s == #items_name[i].0 {
                        return #items_name[i].1 as u32;
                    }
                }
                return 0;
            }
        );
        match_arms.push(arm);

    }

    //let len = items.len();
    //let items_stream: Vec<TokenStream> = items.into_iter().map(|g| g.stream().into()).collect();
    //
    //let items_name_base = format!("{}_ITEMS", function_name);
    //let items_name = proc_macro2::Ident::new(&items_name_base, span);
    //// could use group.delimiter to reproduce user's grouping...
    //let items_decl = quote!(
    //    const #items_name: [(#type_info); #len] = [#((#items_stream)),*];
    //);
    //output.push(items_decl.into());

    let func_name = proc_macro2::Ident::new(&function_name, span);
    let func_def = quote!(
        fn #func_name(s: &str) -> u32 {
            match s.len() {
                #(#match_arms),*
                _ => 0
            }
        }
    );
    output.push(func_def.into());

    // and return the collected streams
    //output.into_iter().map(|s| s.into()).collect::<proc_macro::TokenStream>()
    output.into_iter().map(|s| -> proc_macro::TokenStream { s.into() }).collect()
}

///
/// capture the groups, each of which defines an element that is to be searched.
///
fn build_item_list(items: TokenStream) -> Vec<proc_macro2::Group> {
    let mut item_groups: Vec<proc_macro2::Group> = Vec::new();
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

/// group the items into buckets by length of the LitStr
fn group_items(buckets: &mut FxHashMap::<usize, Vec<proc_macro2::Group>>, items: Vec<proc_macro2::Group>) {
    for item in items {
        let tokens = item.stream().into_iter().collect::<Vec<_>>();
        match &tokens[0] {
            TokenTree::Literal(str_value) => {
                let str = str_value.to_string();
                // get rid of the quotes
                let str = &str[1..str.len() - 1];
                let len = str.len();
                let vec = buckets.entry(len).or_insert(Vec::<proc_macro2::Group>::new());
                vec.push(item);

            },
            _ => panic!("first item must be string literal")
        }
    }
}
