use std::io::prelude::*;
use std::fs::File;
use std::io;
use std::iter::IteratorExt;
use regex::Regex;

pub fn read_input_file(filename: &String) -> io::Result<Vec<(i32, i32)>> {

    let mut file = try!(File::open(filename));

    let mut content = String::new();
    try!(file.read_to_string(&mut content));

    let re = regex!(r"\s*(?P<x>\d+)\s+(?P<y>\d+)");

    let mut points = Vec::new();

    for mut line in content.split_str("\n").skip(1) {
        if line.len() == 0 {
            break; // end of file
        }
        let coords = re.captures(line).and_then(|caps| {
            caps.name("x").and_then(|x_str| x_str.parse().ok()).and_then(|x| {
                caps.name("y").and_then(|y_str| y_str.parse().ok()).map(|y| (x, y))
            })
        }).expect(&format!("Failed to parse line '{}'", line));
        points.push(coords);
    }
    Ok(points)
}
