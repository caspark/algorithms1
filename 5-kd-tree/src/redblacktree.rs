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
    // in practice, Key is always a Some while the RedBlackTree is "at rest"; however, during a function call, we use
    // Option's take() to take ownership of `key` from an &mut because other we cannot guarantee K has a zero value
    // for use with mem::replace() in another way.
    key: Option<K>,
    value: Option<V>, // same as for Key: this should always be a Some when the tree is "at rest".
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    color: Color,
    n: i32, // subtree count
}

impl<K, V> Node<K, V> {
    /// Convenience method to get a reference to the key
    fn key(&self) -> &K {
        self.key.as_ref().expect("Key is expected to be present in a node")
    }

    /// Convenience method to get a reference to the value
    fn value(&self) -> &V {
        self.value.as_ref().expect("Value is expected to be present in a node")
    }

    // /// Invalidates this node (removes the key and value)
    // fn take(&mut self) -> Node {
    //
    // }
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

impl<'a, K, V> RedBlackTree<K, V> where K: Ord {
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

    fn get_from_node(mut node: Option<&'a Box<Node<K, V>>>, key: &K) -> Option<&'a V> {
        loop {
            node = match node {
                None => return None,
                Some(ref curr_node) => {
                    match key.cmp(curr_node.key()) {
                        Ordering::Less => curr_node.left.as_ref(),
                        Ordering::Equal => return Some(&curr_node.value()),
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
        let mut new_root = RedBlackTree::put_in_node(self.root.as_mut(), key, val);
        new_root.color = Color::Black;
        self.root = Some(Box::new(new_root));
    }

    fn put_in_node(maybe_node: Option<&mut Box<Node<K, V>>>, key: K, val: V) -> Node<K, V> {
        match maybe_node {
            None => Node {
                key: Some(key),
                value: Some(val),
                left: None,
                right: None,
                color: Color::Red,
                n: 1,
            },
            Some(mut node) => {
                match key.cmp(node.key()) {
                    Ordering::Less => {
                        node.left = Some(Box::new(RedBlackTree::put_in_node(node.left.as_mut(), key, val)))
                    },
                    Ordering::Equal => node.value = Some(val),
                    Ordering::Greater => node.right = Some(Box::new(RedBlackTree::put_in_node(node.right.as_mut(), key, val))),
                };

                //HACK: we can't move `node` because we've only borrowed it, so construct a new node which is the same
                // Perhaps we can work around this by taking ownership in the function, by having the root store
                // an Option which can be take()n at the top level - that should be more performant.
                let mut node = Node {
                    key: node.key.take(),
                    value: node.value.take(),
                    left: node.left.take(),
                    right: node.right.take(),
                    color: node.color,
                    n: node.n,
                };

                //fix up any right leaning links
                if is_red(node.right.as_ref()) && !is_red(node.left.as_ref()) {
                    node = RedBlackTree::rotate_left(node);
                }
                if is_red(node.left.as_ref()) && is_red(node.left.as_ref().expect("Must have a left node because it's red").left.as_ref()) {
                    node = RedBlackTree::rotate_right(node);
                }
                if is_red(node.left.as_ref()) && is_red(node.right.as_ref()) {
                    RedBlackTree::flip_colors(&mut node);
                }
                node.n = size(node.left.as_ref()) + size(node.right.as_ref()) + 1;

                node
            },
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
}


#[cfg(test)]
mod tests {
    use super::RedBlackTree;

    #[test]
    fn size_and_empty() {
        let t = RedBlackTree::<(), ()>::new();

        assert!(t.is_empty());
        assert_eq!(t.size(), 0);

        //TODO add an item and check the size & empty status changes
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
}