#![feature(plugin)]
#![plugin(regex_macros)]

#[macro_use]
extern crate mdo;
extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate regex;

use std::env;
use point::Point;

mod drawing;
mod parser;
mod point;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error: missing filename argument: should be the path to a plain text input file");
        return;
    }
    let filename = &args[1];
    println!("Should now open {}", filename);

    let points = parser::read_input_file(filename).unwrap().map_in_place(|(x, y)| Point::new(x, y));
    for p in points {
        println!("Point: {:?}", p);
    }

    drawing::display();
}
