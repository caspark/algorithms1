use point::Point;
use std::sync::mpsc::Sender;
use std::cmp;

pub fn find_all_lines(points: &[Point], line_sender: &Sender<Option<[i32; 4]>>) {
    for p1 in points {
        for p2 in points {
            let line = [p1.x, p1.y, p2.x, p2.y];
            line_sender.send(Some(line)).unwrap();
        }
    }
}

pub fn find_colinear_points_slow(points: &[Point], line_sender: &Sender<Option<[i32; 4]>>) {
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
                        let line = line_from_points(&mut [p1.clone(), p2.clone(), p3.clone(), p4.clone()]);
                        line_sender.send(Some(line)).unwrap();
                    }
                }
            }
        }
    }
}

fn line_from_points(points: &mut [Point]) -> [i32; 4] {
    points.sort();
    let last = points.len() - 1;
    [points[0].x, points[0].y, points[last].x, points[last].y]
}

pub fn find_colinear_points_fast(points: &[Point], line_sender: &Sender<Option<[i32; 4]>>) {
    let mut sortable_points = points.to_vec();
    for origin in points {
        sortable_points.sort_by(|ref a, ref b| origin.cmp_by_relative_slope(a, b));
        // println!("Origin is now {:?}; iterating over {:?}", origin, sortable_points);

        let mut first = 0;
        loop {
            if first == sortable_points.len() {
                break;
            }

            let slope_to_first = origin.slope_to(&sortable_points[first]);
            let mut last = first + 1;
            let mut min = origin;
            let mut max = origin;
            while last != sortable_points.len() && slope_to_first == origin.slope_to(&sortable_points[last]) {
                if sortable_points[last].cmp(min) == cmp::Ordering::Less {
                    min = &sortable_points[last];
                }
                if sortable_points[last].cmp(max) == cmp::Ordering::Greater {
                    max = &sortable_points[last];
                }

                last += 1;
            }
            if last - first >= 3 // want at least 4 (including origin) points on a line segment
                    && min == origin { // avoid reporting the same line more than once
                let line = [min.x, min.y, max.x, max.y];
                // println!("Sending line {:?}", line);
                line_sender.send(Some(line)).unwrap();
            }

            first = last;
        }
    }
}
