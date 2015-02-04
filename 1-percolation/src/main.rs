#![feature(rand)] // so we can use random numbers without warnings
#![feature(core)] // otherwise we get a warning from generated code of #[derive(Debug)]

extern crate quickcheck;

mod conversions;
mod unionfind;
mod percolation;

fn main() {
    // use conversions::TryU32Converter;
    // use std::num::Int;
    // println!("Going to convert a too-big usize to a u32 (should panic)");
    // (2us.pow(32)).try_u32();

    use percolation::Percolation;
    let mut perc = Percolation::new(10);

    assert!(!perc.is_open(1, 1));

    perc.open(1, 1);
    assert!(perc.is_open(1, 1));
    assert!(perc.is_open(2, 1));
    assert!(perc.is_open(1, 2));
}
