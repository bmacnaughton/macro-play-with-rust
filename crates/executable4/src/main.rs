

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
            ("klo", Index::Klo)
        ];
    );

    for s in ["bruce", "klo", "zane", "other"] {
        println!("lookup {}: {:?}", s, xyzzy(s));
    }
}

