//@ revisions: edition2021 edition2024
//@ [edition2024] compile-flags: -Zunstable-options
//@ [edition2024] edition: 2024
//@ [edition2024] run-pass

#![cfg_attr(edition2024, feature(shorter_tail_lifetimes))]

fn f() -> usize {
    let c = std::cell::RefCell::new("..");
    c.borrow().len() //[edition2021]~ ERROR: `c` does not live long enough
}

fn main() {
    let _ = f();
}
