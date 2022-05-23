// necessary for the TokenStream::from_str() implementation
use std::str::FromStr;

// for not-newline terminated output
use std::io;
// flush needs the Write trait...
use std::io::Write;

// display macro expansion:
// $ rustup run nightly cargo rustc -- -Zunpretty=expanded
// https://stackoverflow.com/questions/28580386/how-do-i-see-the-expanded-macro-code-thats-causing-my-compile-error
use proc_macro2:: {TokenStream, TokenTree};

use mac::{
    build_thing
};

#[derive(Debug, Copy, Clone)]
enum QueryType {
    /// Mongo Object Expansion Only
    Gte = 1,
    Gt,
    Lt,
    Lte,
    Eq,
    Ne,
    In,
    Nin,
    Mod,
    All,
    Size,
    Exists,
    Type,
    Slice,
    Or,

    /// Mongo String Injection and Object Expansion
    Where,

    /// Mongo String Injection Only
    MapReduce,
    Reduce,
    Finalize,
    KeyF,
    Body,
    Accumulator,
    Init,
    Merge,
    Accumulate,
}
//
build_thing!(get_query_type, (&'static str, QueryType), [
    ("$gte", QueryType::Gte),
    ("$gt", QueryType::Gt),
    ("$lt", QueryType::Lt),
    ("$lte", QueryType::Lte),
    ("$eq", QueryType::Eq),
    ("$ne", QueryType::Ne),
    ("$in", QueryType::In),
    ("$nin", QueryType::Nin),
    ("$mod", QueryType::Mod),
    ("$all", QueryType::All),
    ("$size", QueryType::Size),
    ("$exists", QueryType::Exists),
    ("$type", QueryType::Type),
    ("$slice", QueryType::Slice),
    ("$or", QueryType::Or),

    ("$where", QueryType::Where),

    ("mapReduce", QueryType::MapReduce),
    ("$reduce", QueryType::Reduce),
    ("$finalize", QueryType::Finalize),
    ("$keyf", QueryType::KeyF),
    ("body", QueryType::Body),
    ("accumulator", QueryType::Accumulator),
    ("init", QueryType::Init),
    ("merge", QueryType::Merge),
    ("accumulate", QueryType::Accumulate),
]);

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim_end();
        if line.len() == 0 {
            break;
        }
        if line.len() > 5 && &line[0..5] == "/show" {
            let stream: TokenStream = TokenStream::from_str(&line[5..]).unwrap().into();
            show_token_stream(stream);
            continue;
        }
        println!("executing get_query_type(\"{}\") -> {}", line, get_query_type(line));
    }
}

pub fn show_token_stream(input: TokenStream) {
    display_token_stream(input.into(), 0);
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
