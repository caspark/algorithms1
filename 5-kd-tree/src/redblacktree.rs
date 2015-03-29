use std::cmp::{Ord, Ordering};
use std::mem;

#[derive(Debug, Eq, PartialEq, Copy)]
enum Color {
    Red,
    Black,
}

impl Color {
    /// If red, change to black. If black, change to red.
    fn invert(&mut self) {
        let opposite = self.opposite();
        mem::replace(self, opposite);
    }

    /// If red, return black. If black, return red.
    fn opposite(&self) -> Color {
        match self {
            &Color::Red => Color::Black,
            &Color::Black => Color::Red,
        }
    }
}

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    color: Color,
    n: i32, // size of this subtree
}

fn is_red<K, V>(maybe_node: Option<&Box<Node<K, V>>>) -> bool {
    match maybe_node {
        None => false,
        Some(node) => node.color == Color::Red,
    }
}

fn size<K, V>(maybe_node: Option<&Box<Node<K, V>>>) -> i32 {
    match maybe_node {
        None => 0,
        Some(node) => node.n,
    }
}

struct RedBlackTree<K, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<'t, K, V> RedBlackTree<K, V> where K: Ord {
    pub fn new() -> RedBlackTree<K, V> {
        RedBlackTree {
            root: None
        }
    }

    pub fn size(&self) -> i32 {
        size(self.root.as_ref())
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        RedBlackTree::get_from_node(self.root.as_ref(), key)
    }

    fn get_from_node(mut node: Option<&'t Box<Node<K, V>>>, key: &K) -> Option<&'t V> {
        loop {
            node = match node {
                None => return None,
                Some(ref curr_node) => {
                    match key.cmp(&curr_node.key) {
                        Ordering::Less => curr_node.left.as_ref(),
                        Ordering::Equal => return Some(&curr_node.value),
                        Ordering::Greater => curr_node.right.as_ref(),
                    }
                },
            }
        };
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn put(&mut self, key: K, val: V) {
        let mut new_root = RedBlackTree::put_in_node(self.root.take(), key, val);
        new_root.color = Color::Black;
        self.root = Some(new_root);
    }

    fn put_in_node(maybe_node: Option<Box<Node<K, V>>>, key: K, val: V) -> Box<Node<K, V>> {
        match maybe_node {
            None => Box::new(Node {
                key: key,
                value: val,
                left: None,
                right: None,
                color: Color::Red,
                n: 1,
            }),
            Some(mut node) => {
                match key.cmp(&node.key) {
                    Ordering::Less => node.left = Some(RedBlackTree::put_in_node(node.left.take(), key, val)),
                    Ordering::Equal => node.value = val,
                    Ordering::Greater => node.right = Some(RedBlackTree::put_in_node(node.right.take(), key, val)),
                };

                //fix up any right leaning links
                if is_red(node.right.as_ref()) && !is_red(node.left.as_ref()) {
                    node = Box::new(RedBlackTree::rotate_left(*node));
                }
                if is_red(node.left.as_ref()) && is_red(node.left.as_ref().expect("left node is red -> it exists").left.as_ref()) {
                    node = Box::new(RedBlackTree::rotate_right(*node));
                }
                if is_red(node.left.as_ref()) && is_red(node.right.as_ref()) {
                    RedBlackTree::flip_colors(&mut node);
                }
                node.n = size(node.left.as_ref()) + size(node.right.as_ref()) + 1;

                node
            },
        }
    }

    //TODO it'd be nice to return the deleted element here, which also avoids panicking when the tree is empty
    pub fn delete_min(&mut self) {
        let mut taken_root = self.root.take().expect("Tree should not be empty");
        if !is_red(taken_root.left.as_ref()) && !is_red(taken_root.right.as_ref()) {
            taken_root.color = Color::Red;
        }

        self.root = RedBlackTree::delete_min_node(taken_root);
        if !self.is_empty() { //TODO map instead
            self.root.as_mut().expect("tree is known to be non-empty").color = Color::Black;
        }
        //TODO implement check_state()
        // assert!(self.check_state());
    }

    fn delete_min_node(mut h: Box<Node<K, V>>) -> Option<Box<Node<K, V>>> {
        if h.left.is_none() {
            None
        } else {
            if !is_red(h.left.as_ref()) && !is_red(h.left.as_ref().expect("left node is red -> it exists").left.as_ref()) {
                h = Box::new(RedBlackTree::move_red_left(*h));
            }

            h.left = RedBlackTree::delete_min_node(h.left.take().expect("h.left should exist"));
            Some(Box::new(RedBlackTree::balance(*h)))
        }
    }

    /// make a left-leaning link lean to the right
    fn rotate_left(mut h: Node<K, V>) -> Node<K, V> {
        assert!(is_red(h.right.as_ref()));

        let mut x = *h.right.take().expect("h must have a right node because we checked that it's red");
        h.right = x.left.take();
        x.left = Some(Box::new(h));
        { // restrict scope of borrow
            let h_pm = x.left.as_mut().expect("x must have a left child because we just set it");
            x.color = h_pm.color;
            h_pm.color = Color::Red;
            x.n = h_pm.n;
            h_pm.n = size(h_pm.left.as_ref()) + size(h_pm.right.as_ref()) + 1;
        }
        x
    }

    /// make a right-leaning link lean to the left
    fn rotate_right(mut h: Node<K, V>) -> Node<K, V> {
        assert!(is_red(h.left.as_ref()));

        let mut x = *h.left.take().expect("h must have a left node because we checked that it's red");
        h.left = x.right.take();
        x.right = Some(Box::new(h));
        { // restrict scope of borrow
            let h_pm = x.right.as_mut().expect("x must have a right child because we just set it");
            x.color = h_pm.color;
            h_pm.color = Color::Red;
            x.n = h_pm.n;
            h_pm.n = size(h_pm.left.as_ref()) + size(h_pm.right.as_ref()) + 1;
        }
        x
    }

    /// flip the colors of a node and its two children
    fn flip_colors(h: &mut Node<K, V>) {
        // assert that h has opposite color of its two children
        assert!(h.left.is_some(), "flip_colors: h must have 2 children but is missing a left child");
        assert!(h.right.is_some(), "flip_colors: h must have 2 children but is missing a right child");
        assert!(is_red(h.left.as_ref()) == is_red(h.right.as_ref()), "flip_colors: h's left & right children must be the same colour");
        assert!(h.color != h.left.as_ref().unwrap().color, "flip_colors: h must be of opposite colour to its children");

        h.color.invert();
        h.left.as_mut().unwrap().color.invert();
        h.right.as_mut().unwrap().color.invert();
    }

    /// Assuming that h is red and both h.left and h.left.left are black, make h.left or one of its children red.
    fn move_red_left(mut h: Node<K, V>) -> Node<K, V> {
        assert!(h.color == Color::Red, "h must be red");
        assert!(!is_red(h.left.as_ref()));
        assert!(!is_red(h.left.as_ref().expect("h.left should exist").left.as_ref()));

        RedBlackTree::flip_colors(&mut h);
        if is_red(h.right.as_ref().expect("h.right should exist").left.as_ref()) {
            h.right = Some(Box::new(RedBlackTree::rotate_right(*h.right.expect("h.right should exist"))));
            h = RedBlackTree::rotate_left(h);
            RedBlackTree::flip_colors(&mut h);
        }
        h
    }

    /// Restore red-black tree invariant
    fn balance(mut h: Node<K, V>) -> Node<K, V> {
        if is_red(h.right.as_ref()) {
            h = RedBlackTree::rotate_left(h);
        }
        if is_red(h.left.as_ref()) && is_red(h.left.as_ref().expect("h.left is red -> it exists").left.as_ref()) {
            h = RedBlackTree::rotate_right(h);
        }
        if is_red(h.left.as_ref()) && is_red(h.right.as_ref()) {
            RedBlackTree::flip_colors(&mut h);
        }
        h.n = size(h.left.as_ref()) + size(h.right.as_ref()) + 1;
        h
    }
}


#[cfg(test)]
mod tests {
    use super::RedBlackTree;
    use quickcheck::quickcheck;

    #[test]
    fn size_and_empty() {
        let mut t = RedBlackTree::<i32, i32>::new();

        assert!(t.is_empty());
        assert_eq!(t.size(), 0);

        t.put(1, -1);
        assert!(!t.is_empty());
        assert_eq!(t.size(), 1);
        t.put(2, -2);
        assert!(!t.is_empty());
        assert_eq!(t.size(), 2);
    }

    #[test]
    fn get_and_contains() {
        let mut t = RedBlackTree::<String, i32>::new();

        assert!(!t.contains(&"MyStr".to_string()));
        assert_eq!(t.get(&"MyStr".to_string()), None);

        t.put("One".to_string(), 1);
        t.put("Two".to_string(), 2);
        t.put("Three".to_string(), 3);

        assert!(t.contains(&"One".to_string()));
        assert_eq!(t.get(&"Two".to_string()), Some(&2));
    }

    #[test]
    fn put_various_values() {
        fn prop(mut xs: Vec<i32>) -> bool {
            use rand::{thread_rng, Rng};

            let mut t = RedBlackTree::<i32, String>::new();

            for i in xs.clone() {
                t.put(i, format!("Num {}", i));
            }

            thread_rng().shuffle(&mut xs);
            for i in xs {
                if t.get(&i) != Some(&format!("Num {}", i)) {
                    return false;
                }
            }

            true
        }
        quickcheck(prop as fn(Vec<i32>) -> bool);
    }

    #[test]
    fn delete_min() {
        fn prop(mut xs: Vec<i32>) -> bool {
            //TODO discard property value instead of returning tree
            if xs.len() == 0 {
                return true;
            }

            let mut t = RedBlackTree::<i32, String>::new();

            for i in xs.clone() {
                t.put(i, format!("Num {}", i));
            }

            let min = xs.iter().min().expect("xs len is known to be > 0");
            t.delete_min();

            for i in xs.iter() {
                if i == min {
                    if t.contains(&min) {
                        return false;
                    }
                    if t.get(&min) != None {
                        return false;
                    }
                } else {
                    if t.get(&i) != Some(&format!("Num {}", i)) {
                        return false;
                    }
                }
            }

            true
        }
        quickcheck(prop as fn(Vec<i32>) -> bool);
    }
}
