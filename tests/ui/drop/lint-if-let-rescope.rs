//@ run-rustfix

#![deny(if_let_rescope)]
#![feature(if_let_rescope)]
#![allow(irrefutable_let_patterns)]

fn droppy() -> Droppy {
    Droppy
}
struct Droppy;
impl Drop for Droppy {
    fn drop(&mut self) {
        println!("dropped");
    }
}
impl Droppy {
    fn get(&self) -> Option<u8> {
        None
    }
}

fn main() {
    if let Some(_value) = droppy().get() {
        // Should not lint
    }

    if let Some(_value) = droppy().get() {
        //~^ ERROR: `if let` assigns a shorter lifetime since Edition 2024
        //~| WARN: this changes meaning in Rust 2024
        //~| WARN: this changes meaning in Rust 2024
        //~| ERROR a `match` with a single arm can preserve the drop order up to Edition 2021
        //~| HELP: rewrite this `if let` into `match`
        //~| HELP: rewrite this `if let` into `match`
        // do something
    } else {
        //~^ HELP: the value is now dropped here in Edition 2024
        //~| HELP: rewrite this `if let` into `match`
        // do something else
    }
    //~^ HELP: rewrite this `if let` into `match`

    if let Some(_value) = droppy().get() {
        //~^ ERROR: `if let` assigns a shorter lifetime since Edition 2024
        //~| WARN: this changes meaning in Rust 2024
        //~| WARN: this changes meaning in Rust 2024
        //~| ERROR a `match` with a single arm can preserve the drop order up to Edition 2021
        //~| HELP: rewrite this `if let` into `match`
        //~| HELP: rewrite this `if let` into `match`
        // do something
    } else if let Some(_value) = droppy().get() {
        //~^ HELP: the value is now dropped here in Edition 2024
        //~| ERROR: `if let` assigns a shorter lifetime since Edition 2024
        //~| WARN: this changes meaning in Rust 2024
        //~| HELP: rewrite this `if let` into `match`
        //~| HELP: rewrite this `if let` into `match`
        //~| HELP: rewrite this `if let` into `match`
        // do something else
    }
    //~^ HELP: rewrite this `if let` into `match`
    //~| HELP: the value is now dropped here in Edition 2024

    if droppy().get().is_some() {
        // Should not lint
    } else if let Some(_value) = droppy().get() {
        //~^ ERROR: `if let` assigns a shorter lifetime since Edition 2024
        //~| ERROR a `match` with a single arm can preserve the drop order up to Edition 2021
        //~| WARN: this changes meaning in Rust 2024
        //~| WARN: this changes meaning in Rust 2024
        //~| HELP: rewrite this `if let` into `match`
        //~| HELP: rewrite this `if let` into `match`
    } else if droppy().get().is_none() {
        //~^ HELP: the value is now dropped here in Edition 2024
        //~| HELP: rewrite this `if let` into `match`
    }
    //~^ HELP: rewrite this `if let` into `match`

    if let Some(1) = { if let Some(_value) = Droppy.get() { Some(1) } else { None } } {
        //~^ ERROR: `if let` assigns a shorter lifetime since Edition 2024
        //~| ERROR a `match` with a single arm can preserve the drop order up to Edition 2021
        //~| WARN: this changes meaning in Rust 2024
        //~| WARN: this changes meaning in Rust 2024
        //~| HELP: rewrite this `if let` into `match`
        //~| HELP: rewrite this `if let` into `match`
        //~| HELP: rewrite this `if let` into `match`
        //~| HELP: rewrite this `if let` into `match`
        //~| HELP: the value is now dropped here in Edition 2024
    }

    if let () = { if let Some(_value) = Droppy.get() {} } {
        //~^ ERROR: `if let` assigns a shorter lifetime since Edition 2024
        //~| ERROR a `match` with a single arm can preserve the drop order up to Edition 2021
        //~| WARN: this changes meaning in Rust 2024
        //~| WARN: this changes meaning in Rust 2024
        //~| HELP: rewrite this `if let` into `match`
        //~| HELP: rewrite this `if let` into `match`
        //~| HELP: rewrite this `if let` into `match`
        //~| HELP: the value is now dropped here in Edition 2024
    }
}
