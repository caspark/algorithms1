use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::IteratorExt;

pub fn read_input_file(filename: &String) -> io::Result<Vec<(i32, i32)>> {
    let mut file = try!(File::open(filename));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));

    let re = regex!(r"\s*(?P<x>\d+)\s+(?P<y>\d+)");
    let mut points = Vec::new();
    for line in content.split("\n").skip(1) {
        if line.len() == 0 {
            break; // end of file
        }

        let coords = {
            use mdo::option::{bind, ret};
            mdo! {
                caps =<< re.captures(line).as_ref();
                x_str =<< caps.name("x");
                x =<< x_str.parse().ok();
                y_str =<< caps.name("y");
                y =<< y_str.parse().ok();
                ret ret((x, y))
            }
        };
        points.push(coords.expect(&format!("Failed to parse line '{}'", line)));
    }
    Ok(points)
}
