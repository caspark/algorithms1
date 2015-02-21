use point::Point;
use std::sync::mpsc::Sender;

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



//FIXME this reports lines of size 3 (instead of 4) but gives the appearance of still working because it reports line subsegments twice
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
            while last != sortable_points.len() && slope_to_first == origin.slope_to(&sortable_points[last]) {
                last += 1;
            }
            handle_possible_line(first, last, &sortable_points, line_sender);

            first += 1;
        }
    }

    fn handle_possible_line(first: usize, last: usize, sortable_points: &[Point], line_sender: &Sender<Option<[i32; 4]>>) {
        if last - first >= 3 {
            let found_line = &sortable_points[first..last];

            let line = {
                //FIXME this copying approach is clearly a hack
                let mut v = Vec::with_capacity(last - first);
                for p in found_line {
                    v.push(p.clone());
                }
                v.sort();
                let last = v.len() - 1;
                [v[0].x, v[0].y, v[last].x, v[last].y]
            };
            // println!("Sending line {:?} from points {:?}", line, found_line);
            line_sender.send(Some(line)).unwrap();
        }
    }
}
