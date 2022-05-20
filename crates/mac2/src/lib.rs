use rustc_hash::FxHashMap;

// necessary for the TokenStream::from_str() implementation
use std::str::FromStr;

extern crate proc_macro;

use proc_macro::{
    TokenStream,
    TokenTree,
    //Ident,
};
//use proc_macro2:: {
//    Span,
//};
use quote::{format_ident, quote};
use syn::{
    ItemStruct,
    parenthesized,
    bracketed,
    braced,
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
    Lit, LitStr, Field,
};

//#[proc_macro]
//pub fn show_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//    let input = proc_macro2::TokenStream::from(input);
//    let ast: syn::Item = syn::parse2(input).unwrap();
//    //let ast: syn::Item = syn::parse_macro_input!(input).unwrap();
//    println!("{:?}", ast);
//    let xyzzy = format_ident!("{}", "xyzzy");
//    let output: proc_macro::TokenStream = {
//        // transform input
//        quote!(
//            fn #xyzzy() -> u8 {
//                1u8
//            }
//            #xyzzy
//        ).into()
//    };
//    //quote!(1)
//    output
//    //proc_macro::TokenStream::from(input)
//}

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

struct SignatureSyntax {
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
        let syntax = SignatureSyntax {
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

//#[proc_macro]
//pub fn make_function(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//    // remove this line and rename _input => input to use macro args as token
//    // stream.
//    let input = TokenStream::from_str("fn double(usize) -> usize").unwrap().into();
//    let signature = syn::parse_macro_input!(input as Signature);
//    let function = signature.function;
//    let arguments = signature.arguments;
//    let return_t = signature.return_t;
//
//    println!("sig.func {:?}", function);
//    println!("sig.args {:?}", arguments);
//    println!("sig.ret {:?}", return_t);
//
//    if let 1 = arguments.len()  {
//        let arg = &arguments[0];
//        let tokens = quote!{
//            fn #function(arg: #arg) -> #return_t {
//                let ret: #return_t = arg * 2;
//                println!("input {} * 2 = {}", arg, ret);
//                ret
//            }
//        };
//        tokens.into()
//    } else {
//        panic!("Invalid input");
//    }
//}


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

    let array: u32;
    match &inputs[4] {
        TokenTree::Group(g) => {
            array = build_item_list(g.stream(), type_info);
            // iterative parsing
        },
        _ => panic!("items expected")
    }


    let span = proc_macro2::Span::call_site();
    let func_name = proc_macro2::Ident::new(&function_name, span);
    let func_def = quote!(
        fn #func_name() -> usize {
            #array as usize
        }
    );
    output.push(func_def.into());

    // and return the collected streams
    output.into_iter().collect()
}

fn build_item_list(items: TokenStream, type_info: TokenStream) -> u32 {
    let mut output: Vec<TokenStream> = Vec::new();
    let mut item_groups: Vec<proc_macro::Group> = Vec::new();
    #[derive(PartialEq)]
    enum State {
        NeedGroup,
        MaybeComma,
    }

    let mut state = State::NeedGroup;
    let mut item_count = 0u32;
    for item in items {
        match item {
            TokenTree::Group(g) => {
                if state != State::NeedGroup {
                    panic!("missing item");
                }
                item_count += 1;
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
    println!("found {} items", item_groups.len());

    //output.into_iter().collect()
    item_count
}

// using SignatureSyntax pattern as a template
//#[derive(Debug)]
//struct WordsList {
//    words: Vec<LitStr>,
//}

//struct WordsListSyntax {
//    _bracket: token::Bracket,
//    //words: Punctuated<LitStr, Token![,]>,
//    words: TokenStream,
//}
//
//impl Parse for WordsList {
//    fn parse(input: ParseStream) -> Result<Self> {
//        if input.is_empty() {
//            panic!("Missing bracketed word list, e.g., [\"bruce\"]");
//        }
//
//        let content;
//        let syntax = WordsListSyntax {
//            _bracket: bracketed!(content in input),
//            words: content::<Punctuated::<LitStr, Token![,]>>.parse()?,
//            //words: content.parse_body_with(Punctuated::<LitStr, Token![,]>::parse_terminated),
//            //words: content.parse_terminated(LitStr::parse)? ,
//        };
//
//        println!("words after parse: {:?}", syntax.words);
//
//        Ok(WordsList {
//            //words: syntax.words.into_iter().collect(),
//            words: Vec::new(),
//        })
//    }
//}

#[proc_macro]
pub fn words_list(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let inputs = input.clone().into_iter().collect::<Vec<_>>();
    let mut strings = Vec::<String>::new();

    #[derive(PartialEq)]
    enum State {
        NeedIdent,
        NeedComma,
        NeedLiteral,
        MaybeComma,
    }


    let mut state = State::NeedIdent;
    let mut identifier: Option<proc_macro::Ident> = None;
    for input in inputs {
        match input {
            TokenTree::Ident(ident) => {
                if state != State::NeedIdent {
                    panic!("needed identifier");
                }
                identifier = Some(ident);
                state = State::NeedComma;
            },
            TokenTree::Literal(lit) => {
                if state != State::NeedLiteral {
                    panic!("expected literal str");
                }
                let s = lit.to_string();
                // remove the quotes around the quoted string literal
                strings.push(s[1..s.len() - 1].to_string());
                println!("literal thing {:?}", s);
                state = State::MaybeComma;
            },
            TokenTree::Punct(punc) => {
                if state != State::MaybeComma && state != State::NeedComma {
                    panic!("expected comma");
                }
                println!("punct thing {:?}", punc.to_string());
                state = if state == State::NeedComma {
                    State::NeedLiteral
                } else {
                    State::NeedLiteral
                };
            },
            _ => {
                panic!("didn't expect the spanish inquisition");
            }
        }
    }

    let len = strings.len();
    let q = quote!(
        const TEST: [&str; #len] = [#(#strings),*];
    );
    let q2 = quote!(
        const T2: [&str; #len] = [#(#strings),*];
    );

    let mut func_name = Ident::new("get_t2_len", proc_macro2::Span::call_site());
    if let Some(ident) = identifier {
        let ident = ident.to_string();
        println!("the ident string {}", ident);
        func_name = Ident::new(&ident, proc_macro2::Span::call_site());
    }
    let q3 = quote!(
        fn #func_name() -> usize {
            T2.len()
        }
    );
    let stream: [TokenStream; 3] = [q.into(), q2.into(), q3.into()];
    stream.into_iter().collect()
}

/**
 *
 */

//https://docs.rs/syn/latest/syn/macro.parenthesized.html
// Parse a simplified tuple struct syntax like:
//
//     struct S(A, B);
//struct TupleStruct {
//    //struct_token: Token![struct],
//    //ident: Ident,
//    paren_token: token::Paren,
//    fields: Punctuated<Type, Token![,]>,
//    //semi_token: Token![;],
//}
//
//impl Parse for TupleStruct {
//    fn parse(input: ParseStream) -> Result<Self> {
//        let content;
//        Ok(TupleStruct {
//            //struct_token: input.parse()?,
//            //ident: input.parse()?,
//            paren_token: parenthesized!(content in input),
//            fields: content.parse_terminated(Type::parse)?,
//            //semi_token: input.parse()?,
//        })
//    }
//}


//#[proc_macro]
//pub fn tuple_struct () {
//    ()
//}

/**
 *
 *
 *
 */
//
//
#[derive(Debug)]
enum Item {
    Struct(ItemStruct),
    //Enum(ItemEnum),
}

#[derive(Debug)]
struct ItemStruct2 {
    //struct_token: Token![struct],
    //ident: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse()
        //let lookahead = input.lookahead1();
        //if lookahead.peek(Token![struct]) {
        //    input.parse().map(Item::Struct)
        //} else if lookahead.peek(Token![enum]) {
        //    input.parse().map(Item::Enum)
        //} else {
        //    Err(lookahead.error())
        //}
    }
}

impl Parse for ItemStruct2 {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(ItemStruct2 {
            //struct_token: input.parse()?,
            //ident: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse_named)?,
        })
    }
}

#[proc_macro]
pub fn item_struct2(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as Item);

    println!("item struct2: {:?}", input);

    quote!( () ).into()
}
