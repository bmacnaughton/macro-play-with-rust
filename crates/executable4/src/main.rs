

extern crate mac4;

use mac4::make_lookup_by_str_funct;

#[derive(Debug,Copy,Clone)]
enum Index {
    Bruce,
    Zane,
    Klo,
}

// expand
// $ cargo +nightly run -- -Zunpretty=expanded

fn main() {

    make_lookup_by_str_funct!(
        const xyzzy: (&str, Index) = [
            ("bruce", Index::Bruce),
            ("zane", Index::Zane),
            ("chloe", Index::Klo)
        ];
    );

    for s in ["bruce", "chloe", "zane", "other"] {
        println!("lookup {}: {:?}", s, xyzzy(s));
    }

    make_lookup_by_str_funct!(
        const another_phrase: (&str, &str) = [
            ("once", "one time"),
            ("elated", "ecstatic"),
            ("stupid", "dumb"),
        ];
    );


    for s in ["once", "elated", "stupid", "i don't know"] {
        println!("another phrase for {} is {:?}", s, another_phrase(s));
    }

    make_lookup_by_str_funct!(
        const exper: (&str, usize) = [
            "zero",
            "one",
            "two"
        ];
    );

    for s in ["zero", "one", "two", "three"] {
        println!("{}'s index is {:?}", s, exper(s));
    }


}

