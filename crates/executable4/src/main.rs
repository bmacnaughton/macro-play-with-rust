#![deny(clippy::all)]

// execute this to see macro expansion
// cargo +nightly rustc --profile=check -- -Zunpretty=expanded

extern crate mac4;

use mac4::make_lookup_by_str_func;

#[derive(Debug,Copy,Clone)]
enum Index {
    Bruce,
    Zane,
    Klo,
}

// expand
// $ cargo +nightly run -- -Zunpretty=expanded

fn main() {

    make_lookup_by_str_func!(
        fn xyzzy -> (&str, Index) for [
            ("bruce", Index::Bruce),
            ("zane", Index::Zane),
            ("chloe", Index::Klo)
        ];
    );

    for s in ["bruce", "chloe", "zane", "other"] {
        println!("lookup {}: {:?}", s, xyzzy(s));
    }

    make_lookup_by_str_func!(
        fn another_phrase -> (&str, &str) for [
            ("once", "one time"),
            ("elated", "ecstatic"),
            ("evil", "wicked"),
        ];
    );


    for s in ["once", "elated", "evil", "i don't know"] {
        println!("another way to say \"{}\" is {:?}", s, another_phrase(s));
    }

    make_lookup_by_str_func!(
        fn exper -> (&str, i32) for [
            "zero",
            "one",
            "two"
        ];
    );

    for s in ["zero", "one", "two", "three"] {
        println!("{}'s index is {:?}", s, exper(s));
    }

    make_lookup_by_str_func!(
        pub fn another_tuple -> (&str, (u32, &str)) for [
            ("once", (42, "one time")),
            ("elated", (7, "ecstatic")),
            ("evil", (666, "wicked")),
        ];
    );


    for s in ["once", "elated", "evil", "i don't know"] {
        println!("another way to say {} is {:?}", s, another_tuple(s));
    }


}

