use std::cmp;
use std::num::Float;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> f64 {
        self.distance_squared_to(other).sqrt()
    }

    pub fn distance_squared_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx.powi(2) + dy.powi(2)
    }
}

impl cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        // yeah, comparing floating point numbers in this way is dodgy, but we're
        // copying how it's done in Point2D.java in the course.
        self.x == other.x && self.y == other.y
    }
}

impl cmp::Eq for Point {}

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
    use std::num::Float;

    #[test]
    fn points_ordering_should_be_by_y_coord_then_by_x_coord() {
        assert_eq!(Point { x: 1.0, y: 1.0 }, Point { x: 1.0, y: 1.0 });
        assert!(Point { x: 1.0, y: 0.0 } != Point { x: 1.0, y: 1.0 });
        assert!(Point { x: 0.0, y: 1.0 } != Point { x: 1.0, y: 1.0 });

        // y should be first priority for compares
        assert!(Point { x: 1.0, y: 0.0 } < Point { x: 1.0, y: 1.0 });
        assert!(Point { x: 1.0, y: 0.0 } < Point { x: 0.0, y: 1.0 });
        assert!(Point { x: 0.0, y: 1.0 } > Point { x: 1.0, y: 0.0 });

        // x should be tie breaker
        assert!(Point { x: 1.0, y: 1.0 } > Point { x: 0.0, y: 1.0 });
        assert!(Point { x: 0.0, y: 1.0 } < Point { x: 1.0, y: 1.0 });
    }

    #[test]
    fn point_distances() {
        assert_eq!(Point { x: 0.0, y: 0.0 }.distance_to(&Point { x: 1.0, y: 1.0 }),
                2f64.sqrt());

        assert_eq!(Point { x: 0.0, y: 0.0 }.distance_squared_to(&Point { x: 1.0, y: 1.0 }),
                2f64);
    }
}
