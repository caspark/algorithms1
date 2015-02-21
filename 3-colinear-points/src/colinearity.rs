use point::Point;
use std::sync::mpsc::Sender;

pub fn find_all_lines(points: &[Point], line_sender: Sender<[i32; 4]>) {
    for p1 in points {
        for p2 in points {
            let line = [p1.x, p1.y, p2.x, p2.y];
            line_sender.send(line).unwrap();
        }
    }
}

pub fn find_colinear_points_slow(points: &[Point], line_sender: Sender<[i32; 4]>) {
    panic!("Slow colinear point finding not implemented yet!");
}

pub fn find_colinear_points_fast(points: &[Point], line_sender: Sender<[i32; 4]>) {
    panic!("Fast colinear point finding not implemented yet!");
}
