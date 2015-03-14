use point::Point;
use std::num::Float;

#[derive(Debug)]
struct Rect([f64; 4]);

impl Rect {
    pub fn new(bounds: [f64; 4]) -> Rect {
        assert!(bounds[0] < bounds[2], "rect x min coord must be < x max coord!");
        assert!(bounds[1] < bounds[3], "rect y min coord must be < y max coord!");

        Rect(bounds)
    }

    // okay so maybe a tuple struct was a poor choice given this mess, but worth a shot
    pub fn xmin(&self) -> f64 {
        match self { &Rect([v, _, _, _]) => v }
    }

    pub fn ymin(&self) -> f64 {
        match self { &Rect([_, v, _, _]) => v }
    }

    pub fn xmax(&self) -> f64 {
        match self { &Rect([_, _, v, _]) => v }
    }

    pub fn ymax(&self) -> f64 {
        match self { &Rect([_, _, _, v]) => v }
    }

    /// same rectangle represented by top-left corner (x, y) & width & height: [x, y, w, h]
    pub fn as_rect_wh(&self) -> [f64; 4] {
        [self.xmin(), self.ymin(), self.xmax() - self.xmin(), self.ymax() - self.ymin()]
    }

    pub fn contains(&self, p: &Point) -> bool {
        p.x >= self.xmin() && p.x <= self.xmax()
                && p.y >= self.ymin() && p.y <= self.ymax()
    }

    pub fn intersects(&self, that: &Rect) -> bool {
        self.xmax() >= that.xmin() && self.ymax() >= that.ymin()
                && that.xmax() >= self.xmin() && that.ymax() >= self.ymin()
    }

    pub fn distance_to(&self, p: &Point) -> f64 {
        self.distance_squared_to(p).sqrt()
    }

    pub fn distance_squared_to(&self, p: &Point) -> f64 {
        let dx = if p.x < self.xmin() { p.x - self.xmin() }
            else if p.x > self.xmax() { p.x - self.xmax() }
            else { 0.0 };
        let dy = if p.y < self.ymin() { p.y - self.ymin() }
            else if p.y > self.ymax() { p.y - self.ymax() }
            else { 0.0 };
        return dx.powi(2) + dy.powi(2);
    }
}


#[cfg(test)]
mod tests {
    use super::Rect;
    use point::Point;

    #[test]
    #[should_panic(expected = "rect x min coord must be < x max coord!")]
    fn invalid_x_rect_cannot_be_constructed() {
        Rect::new([0.0, 0.0, -1.0, 1.0]);
    }

    #[test]
    #[should_panic(expected = "rect y min coord must be < y max coord!")]
    fn invalid_y_rect_cannot_be_constructed() {
        Rect::new([0.0, 0.0, 1.0, -1.0]);
    }

    #[test]
    fn valid_rect_can_be_constructed() {
        Rect::new([0.0, 0.0, 1.0, 1.0]);
    }

    #[test]
    fn as_rect_wh_works() {
        assert_eq!(Rect::new([1.0, 1.0, 3.0, 3.0]).as_rect_wh(), [1.0, 1.0, 2.0, 2.0]);
    }

    #[test]
    fn contains_works() {
        let r = Rect::new([1.0, 1.0, 2.0, 2.0]);

        assert!(r.contains(&Point { x: 1.0, y: 1.0 }));
        assert!(r.contains(&Point { x: 1.0, y: 2.0 }));
        assert!(r.contains(&Point { x: 2.0, y: 1.0 }));
        assert!(r.contains(&Point { x: 2.0, y: 2.0 }));
        assert!(r.contains(&Point { x: 1.5, y: 1.5 }));

        assert!(!r.contains(&Point { x: 0.0, y: 1.5 }));
        assert!(!r.contains(&Point { x: 3.0, y: 1.5 }));
        assert!(!r.contains(&Point { x: 1.5, y: 0.0 }));
        assert!(!r.contains(&Point { x: 1.5, y: 3.0 }));
    }

    #[test]
    fn intersects_works() {
        let r = Rect::new([1.0, 1.0, 2.0, 2.0]);

        assert!(r.intersects(&Rect::new([1.0, 1.0, 2.0, 2.0])));
        assert!(r.intersects(&Rect::new([1.5, 1.5, 3.0, 3.0])));
        assert!(r.intersects(&Rect::new([2.0, 2.0, 3.5, 3.5])));

        assert!(!r.intersects(&Rect::new([-1.0, -1.0, 0.0, 0.0])));
        assert!(!r.intersects(&Rect::new([3.0, 3.0, 4.0, 4.0])));
    }

    #[test]
    fn distances_works() {
        // the cases from the sample diagram
        let r = Rect::new([0.4, 0.3, 0.8, 0.6]);

        assert!(r.distance_to(&Point { x: 0.6, y: 0.5 }) - 0.0 < 0.0000001);
        assert!(r.distance_to(&Point { x: 0.1, y: 0.4 }) - 0.3 < 0.0000001);
        assert!(r.distance_to(&Point { x: 0.0, y: 0.0 }) - 0.5 < 0.0000001);
    }
}
