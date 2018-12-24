# LibHoare

Simple Rust support for design by contract-style assertions. Supports
* preconditions (`precond`),
* postconditions (`postcond`),
* invariants (pre and post)  (`invariant`).

Each macro takes a predicate given as a string parameter. Each macro is
available in a `debug_` version which only checks the assertion in debug builds,
they should be zero overhead in non-debug builds. You can use `result` inside a
postcondition to get the value returned by the function.

Preconditions are checked on entry to a function. Postcondiitons are checked when
leaving the function by any path.

(The library is named for Tony, not Graydon. Or rather it is named for the logic
which was named after Tony).


## Using libhoare

You can use libhoare with Cargo by adding

```
[dependencies.hoare]
git = "https://github.com/badboy/libhoare.git"
branch = "rust-2018"
```

to your projects Cargo manifest.

## Examples:

```
#[hoare(precond="x > 0", postcond="result > 1")]
fn foo(x: int) -> int {
    let y = 45 / x;
    y + 1
}


struct Bar {
    f1: int,
    f2: int
}

#[hoare(invariant="x.f1 < x.f2")]
fn baz(x: &mut Bar) {
    x.f1 += 10;
    x.f2 += 10;
}

fn main() {
    foo(12);
    foo(26);
    // task '<main>' failed at 'precondition of foo (x > 0)'
    // foo(-3);

    let mut b = Bar { f1: 0, f2: 10 };
    baz(&mut b);
    b.f2 = 100;
    baz(&mut b);
    b.f2 = -5;
    // task '<main>' failed at 'invariant entering baz (x.f1 < x.f2)'
    // baz(&mut b);
}
```

You can use contracts on methods as well as functions, but they are not as well
tested.


## Contents

All the code for checking conditions is in `libhoare`. Currently, there is only
a single file, `lib.rs`.

The `test` directory contains unit tests for the library.

## Building

To build libhoare from the top level of your checked out repo run

```
cargo build
```

## TODO

* add tests to RustCI
* tests for debug_ versions of macros - what is the best way to test this?
* better use of macro stuff? (I'm a total beginner at syntax extensions, this all
could probably be implemented better).
* better spans.
* better names for scopes (`<precondition>` rather than `<quote expansion>`, etc.
These appear in the user-visible error messages, so it would be nice if they could
be informative).

Wish list:

* object invariants (I think this would need compiler support, if it is possible
at all. You would add `[#invariant="..."]` to a struct, enum, etc. and the
invariant would be checked on entering and leaving every method defined in any
impl for the object).
