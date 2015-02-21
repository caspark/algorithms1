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
use std::sync::mpsc::{channel, Sender};
use std::thread;

mod colinearity;
mod drawing;
mod parser;
mod point;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = format!("Usage: {} <input-file> <algorithm>\n\twhere <algorithm> is one of 'fast', 'slow' or 'line-draw-test'", args[0]);
    if args.len() != 3 {
        println!("Error: incorrect number of arguments provided.\n{}", usage);
        return;
    }
    let filename = &args[1];
    println!("Input file is {}", filename);

    let line_finder: fn(&[Point], Sender<[i32; 4]>) =
        if args[2] == "fast" {
            println!("Fast algorithm for finding co-linear points selected");
            colinearity::find_colinear_points_fast
        } else if args[2] == "slow" {
            println!("Slow algorithm for finding co-linear points selected");
            colinearity::find_colinear_points_slow
        } else if args[2] == "line-draw-test" {
            println!("Will not find co-linear points; will instead draw lines between all points");
            colinearity::find_all_lines
        } else {
            println!("Error: unrecognised algorithm '{}'.\n{}", args[2], usage);
            return;
        };

    parser::read_input_file(filename).map(|coords| {
        let points = coords.map_in_place(|(x, y)| Point { x: x, y: y });

        let (tx, rx) = channel();
        let points_for_finding_lines = points.clone();
        thread::spawn(move || {
            line_finder(&points_for_finding_lines[..], tx);
            println!("Done finding lines.");
        });

        drawing::display(&points[..], rx);
    }).unwrap();
}
