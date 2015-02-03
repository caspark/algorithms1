#![feature(rand)] // so we can use random numbers without warnings
#![feature(core)] // otherwise we get a warning from generated code of #[derive(Debug)]

extern crate quickcheck;

mod conversions;
mod unionfind;

fn main() {
    use conversions::ToPrimitive;
    use std::num::Int;

    println!("Going to convert a too-big usize to a u32 (should panic)");
    (2us.pow(32)).assume_u32();
}
