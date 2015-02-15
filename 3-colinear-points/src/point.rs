use std::{cmp, f64};

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point{x: x, y: y}
    }

    fn slope_to(&self, other: &Point) -> f64 {
        if self == other {
            f64::NEG_INFINITY
        } else if self.y == other.y {
            0f64
        } else if self.x == other.x {
            f64::INFINITY
        } else {
            ((other.y - self.y) as f64) / ((other.x - self.x) as f64)
        }
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
    use std::f64;

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

    #[test]
    fn slopes_are_calculated_correctly() {
        // degenerate line segment
        assert_eq!(Point::new(0, 0).slope_to(&Point::new(0, 0)), f64::NEG_INFINITY);

        // horizontal line segment
        assert_eq!(Point::new(0, 0).slope_to(&Point::new(1, 0)), 0f64);
        assert_eq!(Point::new(1, 0).slope_to(&Point::new(0, 0)), 0f64);

        // vertical line segment
        assert_eq!(Point::new(0, 0).slope_to(&Point::new(0, 1)), f64::INFINITY);
        assert_eq!(Point::new(0, 1).slope_to(&Point::new(0, 0)), f64::INFINITY);

        // normal slopes
        assert_eq!(Point::new(0, 0).slope_to(&Point::new(1, 1)), 1f64);
        assert_eq!(Point::new(1, 1).slope_to(&Point::new(0, 0)), 1f64);
        assert_eq!(Point::new(0, 0).slope_to(&Point::new(1, 2)), 2f64);
        assert_eq!(Point::new(1, 2).slope_to(&Point::new(0, 0)), 2f64);
        assert_eq!(Point::new(0, 0).slope_to(&Point::new(2, 1)), 0.5f64);
        assert_eq!(Point::new(2, 1).slope_to(&Point::new(0, 0)), 0.5f64);
    }
}