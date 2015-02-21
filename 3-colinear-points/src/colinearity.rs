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
    for p1 in points {
        for p2 in points {
            if p1 == p2 {
                continue;
            }
            let p1_p2_slope = p1.slope_to(p2);
            for p3 in points {
                if p1 == p3 || p2 == p3 {
                    continue;
                }
                let p1_p3_slope = p1.slope_to(p3);
                if p1_p2_slope != p1_p3_slope {
                    continue;
                }
                for p4 in points {
                    if p1 == p4 || p2 == p4 || p3 == p4 {
                        continue;
                    }
                    let p1_p4_slope = p1.slope_to(p4);
                    if p1_p2_slope == p1_p4_slope {
                        // we have a line!
                        let mut a = [p1, p2, p3, p4];
                        a.sort();
                        line_sender.send([a[0].x, a[0].y, a[3].x, a[3].y]).unwrap();
                    }
                }
            }
        }
    }
}

pub fn find_colinear_points_fast(points: &[Point], line_sender: Sender<[i32; 4]>) {
    panic!("Fast colinear point finding not implemented yet!");
}
