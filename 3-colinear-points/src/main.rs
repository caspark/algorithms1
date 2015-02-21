#![feature(collections)]
#![feature(core)]
#![feature(env)]
#![feature(fs)]
#![feature(io)]
#![feature(plugin)]

#![plugin(regex_macros)]

#[macro_use]
extern crate mdo;

extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

extern crate regex;

use point::Point;
use std::env;
use std::sync::mpsc::channel;
use std::thread;

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

    parser::read_input_file(filename).map(|coords| {
        let points = coords.map_in_place(|(x, y)| Point { x: x, y: y });

        let (tx, rx) = channel();

        let points_clone = points.clone();
        thread::spawn(move || {
            for p1 in &points_clone {
                for p2 in &points_clone {
                    let line = (p1.x, p1.y, p2.x, p2.y);
                    tx.send(line).unwrap();
                }
            }
        });

        drawing::display(&points[..], rx);
    }).unwrap();
}
