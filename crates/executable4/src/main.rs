

extern crate mac4;

use mac4::make_lookup_by_str_funct;

// expand
// $ cargo +nightly run -- -Zunpretty=expanded

fn main() {
    make_lookup_by_str_funct!(
        const xyzzy: [&str] = ["bruce", "zane", "klo"];
    );

    for s in ["bruce", "klo", "zane", "other"] {
        println!("lookup {}: {:?}", s, xyzzy(s));
    }
}

