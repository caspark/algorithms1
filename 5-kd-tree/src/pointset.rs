use redblacktree::RedBlackTree;
use rect::Rect;
use point::Point;

struct PointSet {
    points: RedBlackTree<Point, ()>,
}

impl PointSet {
    pub fn new() -> PointSet {
        PointSet {
            points: RedBlackTree::new(),
        }
    }

    pub fn size(&self) -> i32 {
        self.points.size()
    }

    pub fn insert(&mut self, p: Point) {
        self.points.put(p, ());
    }

    pub fn contains(&self, p: &Point) -> bool {
        self.points.contains(p)
    }

    pub fn range(&self, rect: &Rect) -> Vec<&Point> {
        let mut found = Vec::new();
        for p in self.points.keys() {
            if rect.contains(p) {
                found.push(p);
            }
        }
        found
    }

    pub fn nearest(&self, target: &Point) -> Option<&Point> {
        let mut closest = None;
        for p in self.points.keys() {
            if closest.is_none() {
                closest = Some(p);
            } else if p.distance_squared_to(target) < closest.unwrap().distance_squared_to(target) {
                closest = Some(p);
            }
        }
        closest
    }
}
