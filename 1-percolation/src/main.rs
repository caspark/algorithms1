#![feature(rand)] // so we can use random numbers without warnings
#![feature(core)] // otherwise we get a warning from generated code of #[derive(Debug)]
#![feature(int_uint)] //temporary for util mod

extern crate quickcheck;

mod conversions;
mod unionfind;
