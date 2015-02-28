use std::{cmp, f64};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn slope_to(&self, other: &Point) -> f64 {
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

    pub fn cmp_by_relative_slope(&self, a: &Point, b: &Point) -> cmp::Ordering {
        self.slope_to(a).partial_cmp(&self.slope_to(b)).expect("Slopes should have total ordering")
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;
    use std::cmp::Ordering;

    #[test]
    fn points_ordering_should_be_by_y_coord_then_by_x_coord() {
        assert_eq!(Point { x: 1, y: 1 }, Point { x: 1, y: 1 });
        assert!(Point { x: 1, y: 0 } != Point { x: 1, y: 1 });
        assert!(Point { x: 0, y: 1 } != Point { x: 1, y: 1 });

        // y should be first priority for compares
        assert!(Point { x: 1, y: 0 } < Point { x: 1, y: 1 });
        assert!(Point { x: 1, y: 0 } < Point { x: 0, y: 1 });
        assert!(Point { x: 0, y: 1 } > Point { x: 1, y: 0 });

        // x should be tie breaker
        assert!(Point { x: 1, y: 1 } > Point { x: 0, y: 1 });
        assert!(Point { x: 0, y: 1 } < Point { x: 1, y: 1 });
    }

    #[test]
    fn slopes_are_calculated_correctly() {
        // degenerate line segment
        assert_eq!(Point { x: 0, y: 0 }.slope_to(&Point { x: 0, y: 0 }), f64::NEG_INFINITY);

        // horizontal line segment
        assert_eq!(Point { x: 0, y: 0 }.slope_to(&Point { x: 1, y: 0 }), 0f64);
        assert_eq!(Point { x: 1, y: 0 }.slope_to(&Point { x: 0, y: 0 }), 0f64);

        // vertical line segment
        assert_eq!(Point { x: 0, y: 0 }.slope_to(&Point { x: 0, y: 1 }), f64::INFINITY);
        assert_eq!(Point { x: 0, y: 1 }.slope_to(&Point { x: 0, y: 0 }), f64::INFINITY);

        // normal slopes
        assert_eq!(Point { x: 0, y: 0 }.slope_to(&Point { x: 1, y: 1 }), 1f64);
        assert_eq!(Point { x: 1, y: 1 }.slope_to(&Point { x: 0, y: 0 }), 1f64);
        assert_eq!(Point { x: 0, y: 0 }.slope_to(&Point { x: 1, y: 2 }), 2f64);
        assert_eq!(Point { x: 1, y: 2 }.slope_to(&Point { x: 0, y: 0 }), 2f64);
        assert_eq!(Point { x: 0, y: 0 }.slope_to(&Point { x: 2, y: 1 }), 0.5f64);
        assert_eq!(Point { x: 2, y: 1 }.slope_to(&Point { x: 0, y: 0 }), 0.5f64);
    }

    #[test]
    fn comparing_by_slope_considers_points_on_a_line_equal() {
        let base = Point { x: 1, y: 1 };

        assert_eq!(base.cmp_by_relative_slope(&Point { x: 2, y: 1 }, &Point { x: 2, y: 2 }), Ordering::Less);
        assert_eq!(base.cmp_by_relative_slope(&Point { x: 2, y: 2 }, &Point { x: 2, y: 1 }), Ordering::Greater);

        assert_eq!(base.cmp_by_relative_slope(&Point { x: 2, y: 2 }, &Point { x: 3, y: 3 }), Ordering::Equal);
        assert_eq!(base.cmp_by_relative_slope(&Point { x: 3, y: 3 }, &Point { x: 2, y: 2 }), Ordering::Equal);
    }
}
