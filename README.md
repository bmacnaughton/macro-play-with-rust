# experimental work with rust procedural macros

i wanted to learn how to use procedural macros. i had previously worked with macros
in common bliss and loved their power. then i worked with c, and while it's possible
to do fairly sophisticated things with c macros, i always felt hamstrung.

well, when i first saw rust macros, i felt like i'd met a long-lost friend.

# what to do?

i tinker with code, exploring what works well and what doesn't. and, on a lark, one
day i decided to benchmark two different ways of matching strings. (actually, i
benchmarked four ways total, including using `FxHashMap` and `AhoCorasick`)

the first was the default implementation - a sequence of match arms. rust's `match`
is damn good but i wondered if it was possible to do better, at least in the short
term - trying to stay ahead of rust's optimizer is an endless task.

the second was to group the strings by length and only search the subset that matches
the length of the input string. this turned out to be faster when the list to be
searched is not very short and not all elements are the same length.

but while doing this out of curiousity was worthwhile, it's not the kind of thing that
most programmers (or I, anyway) want to do - manually sorting strings by length and
grouping them, then writing a function to search those strings of the same length as
the input string.

my answer was to use this problem to investigate macros.

# the process

well, i stumbled around a lot. and i'm still sorting out exactly when and where to
use proc_macro vs proc_macro2, though i am starting to have a clue or two. i still
stumble around with assembling token streams when i need to.

the lack of real-world examples in the proc_macro and proc_macro2 documentation is,
well, frustrating. i spent a lot of time just trying various things.

i first got the pair `mac` and `executable` working for my first macro, `build_thing`.
it wasn't very flexible but it was a start.

then came `mac2` and `executable2`. it was really just exploration; there was not a
working macro to be found when i went on to the next step.

`mac3` and `executable3` really helped start to put some pieces together. it started as a
cut/paste from https://github.com/dtolnay/syn/blob/master/examples/lazy-static/lazy-static/src/lib.rs. (thank you dtolnay!)

eventually i moved to the fourth incarnation.

# mac4

`mac4` implements the macro i wanted to. i am pretty confident that there are better ways to
do what it does. and i am absolutely certain that error detection and recovery can be better.
but good-syntax-in => good-syntax-out.

the macro `make_lookup_by_str_func` creates a function that looks up a string slice in a list
of `str` and returns an `Option` to a value associated with each `str` in the list, or `None`
if the needle was not found.

example:

```rust
    make_lookup_by_str_func!(
        const another_phrase: (&str, &str) = [
            ("once", "one time"),
            ("elated", "ecstatic"),
            ("evil", "wicked"),
        ];
    );

    for s in ["once", "elated", "evil", "i don't know"] {
        println!("another way to say \"{}\" is {:?}", s, another_phrase(s));
    }

    // outputs:
    // another way to say "once" is Option("one time")
    // another way to say "elated" is Option("ecstatic")
    // another way to say "evil" is Option("wicked")
    // another way to say "i don't know" is None
```

the syntax that `make_lookup_by_str_func` accepts is:

`const <function-name>: <type-tuple> = [<list-def>, ...];`

where:
- `function-name` a function of this name will be created. it will also be used
as a prefix for additional declarations (to avoid name conflicts).
- `type-tuple` is `(&str, <value-type>)`
- `value-type` is what the function returns
- `list-def` are tuples of `(&str, <value-type>)`

and it creates:
- `fn <function-name>(s: &str) -> Option<value-type>`

as a convenience, it also accepts:

`const <function-name>: <type-tuple> = [<str-list>, ...]`

where:
- `function-name`, `type-tuple` are the same as above
- `value-type` must be an integer type (usize, u16, i32, etc.)
- `str-list` is just a list `str`, e.g., `["bruce", "one", "soma", "hendrix"]`

in which case it will create tuples of `value-type` using the index of each str
in `str-list`.

# try it yourself

clone the repo, then:

```bash
$ cd crates/executable4
$ cargo run
# to see expanded macros (they are only output at compile-time. if there are no
# changes to the source file, it won't be compiled again, so make some change
# in main.rs before executing the following command).
$ cargo +nightly rustc -- -Zunpretty=expanded
```

# about the original exploration

as the word list gets longer, at some point `FxHashMap` will win - it is near
constant cost. but for lists of "reasonable" length, where not all words are
the same length, this macro results in optimal performance.
