#![feature(core)] // otherwise we get a warning from generated code of #[derive(Debug)]

extern crate quickcheck;
extern crate rand;

mod conversions;
mod unionfind;
mod percolation;

fn main() {
    // use percolation::Percolation;
    use percolation;

    let stats = percolation::simulate_multiple(200, 10);
    // println!("{:?}", stats);
    println!("Mean: {}", stats.mean());
}
