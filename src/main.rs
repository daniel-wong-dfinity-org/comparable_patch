use comparable::{Changed, Comparable, I32Change, StringChange};

#[derive(Comparable, Debug, Clone, PartialEq, Eq)]
struct Foo {
    bar: i32,
    baz: String,
}

fn main() {
    let s1 = Foo { bar: 42, baz: "3.14".to_string() };
    let s2 = Foo { bar: 7,  baz: "2.72".to_string() };

    // #[derive(Comparable)] makes calculating a "patch" between
    // two objects is just a matter of calling one method...
    let changes = s1.comparison(&s2);

    let mut s1_cp = s1.clone();
    println!("s1_cp before:\n{:#?}\n", s1_cp);

    let changes = match changes {
        Changed::Unchanged => panic!("no change?!"),
        Changed::Changed(changes) => changes,
    };
    // ... However, the `comparable` crate does not seem to supply
    // a way to apply the patch to an object.[1] However, it is not difficult
    // to "hand-craft" such code (it's just really tedious).
    //
    // [1] https://docs.rs/comparable/latest/comparable/index.html#changes-the-comparablechange-associated-type
    //     There, it says "The primary purpose of a Comparable::Change value is
    //     to compare it to a set of changes you expected to see, so design
    //     choices have been made to optimize for clarity and printing rather
    //     than, say, the ability to transform one value into another by
    //     applying a changeset. This is entirely possible give a dataset and a
    //     change description, but no work has been done to achieve this goal."
    for change in changes {
        match change {
            FooChange::Bar(bar) => {
                let I32Change(before, after) = bar;
                assert_eq!(s1_cp.bar, before);
                s1_cp.bar = after;
            }
            FooChange::Baz(baz) => {
                let StringChange(before, after) = baz;
                assert_eq!(s1_cp.baz, before);
                s1_cp.baz = after;
            }
        }
    }

    // After applying the patch, s1_cp is now == s2, even though it used
    // to be == to s1 when s1_cp first came into being.
    assert_eq!(s2, s1_cp);
    println!("s1_cp after:\n{:#?}", s1_cp);
}
