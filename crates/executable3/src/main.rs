

extern crate mac3;

use mac3::lookup_thing_by_str;

struct Xyzzy {
    i: u32
}

fn main() {
    lookup_thing_by_str!(
        static ref xyzzy: Xyzzy = Xyzzy {i: 42};
    );
}
