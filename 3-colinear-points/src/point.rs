use std::cmp;

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Point {
        Point{x: x, y: y}
    }

    fn slopeTo(that: &Point) -> f64 {
        panic!("Not implemented yet");
    }
}

/// Compares by Y coordinates, breaking ties by X coordinates
impl cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Compares by Y coordinates, breaking ties by X coordinates
impl cmp::Ord for Point {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self == other {
            cmp::Ordering::Equal
        } else if self.y != other.y {
            if self.y < other.y {
                cmp::Ordering::Less
            } else {
                cmp::Ordering::Greater
            }
        } else {
            if self.x < other.x {
                cmp::Ordering::Less
            } else {
                cmp::Ordering::Greater
            }
        }
    }
}

// TODO a second comparator, which compares by slope

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_ordering_should_be_by_y_coord_then_by_x_coord() {
        assert_eq!(Point::new(1, 1), Point::new(1, 1));
        assert!(Point::new(1, 0) != Point::new(1, 1));
        assert!(Point::new(0, 1) != Point::new(1, 1));

        // y should be first priority for compares
        assert!(Point::new(1, 0) < Point::new(1, 1));
        assert!(Point::new(1, 0) < Point::new(0, 1));
        assert!(Point::new(0, 1) > Point::new(1, 0));

        // x should be tie breaker
        assert!(Point::new(1, 1) > Point::new(0, 1));
        assert!(Point::new(0, 1) < Point::new(1, 1));
    }
}
