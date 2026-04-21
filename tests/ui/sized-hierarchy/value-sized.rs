//@ run-pass
#![feature(value_sized)]
use std::fmt::Debug;
use std::marker::ValueSized;

fn size_of(x: &dyn ValueSized) -> usize {
    std::mem::size_of_val(x)
}

fn size_of_debug(x: &(dyn Debug + ValueSized)) -> usize {
    std::mem::size_of_val(x)
}

fn main() {
    assert_eq!(size_of(&()), 0);
    assert_eq!(size_of(&0_u8), 1);
    assert_eq!(size_of(&false), 1);
    assert_eq!(size_of(&1_i16), 2);
    assert_eq!(size_of(&1_u32), 4);
    assert_eq!(size_of(&1_u64), 8);

    assert_eq!(size_of_debug(&()), 0);
    assert_eq!(size_of_debug(&0_u8), 1);
    assert_eq!(size_of_debug(&false), 1);
    assert_eq!(size_of_debug(&1_i16), 2);
    assert_eq!(size_of_debug(&1_u32), 4);
    assert_eq!(size_of_debug(&1_u64), 8);
}
