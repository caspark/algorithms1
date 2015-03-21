use std::cmp::{Ord, Ordering};

#[derive(Debug, Eq, PartialEq)]
enum Color {
    Red,
    Black,
}

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    color: Color,
    n: i32, // subtree count
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

    pub fn get(&self, key: K) -> Option<&V> {
        RedBlackTree::get_from_node(self.root.as_ref(), key)
    }

    fn get_from_node(mut node: Option<&'a Box<Node<K, V>>>, key: K) -> Option<&'a V> {
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

    pub fn contains(&self, key: K) -> bool {
        self.get(key).is_some()
    }

    pub fn put(mut self, key: K, val: V) {
        let mut new_root = RedBlackTree::put_in_node(self.root, key, val);
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
                // if is_red(node.right.as_ref()) && !is_red(node.left.as_ref()) {
                //     node = RedBlackTree::rotate_left(node);
                // }
                // if is_red(node.left.as_ref()) && is_red(node.left.as_ref().expect("Should have a left").left.as_ref()) {
                //     node = RedBlackTree::rotate_right(node);
                // }
                // if is_red(node.left.as_ref()) && is_red(node.right.as_ref()) {
                //     RedBlackTree::flip_colors(node);
                // }
                // node.n = size(node.left.as_ref()) + size(node.right.as_ref()) + 1;

                node
            },
        }
    }

    // fn rotate_left(h: Box<Node<K, V>>) -> Box<Node<K, V>> {
    //     //TODO add assert as in original code
    //     let mut x = h.left.expect("#yolo");
    //     h.left = x.right;
    //     x.right = Some(h);
    //     x.right.map(|ref mut h| {
    //         x.color = h.color;
    //         h.color = Color::Red;
    //         x.n = h.n;
    //         h.n = size(h.left.as_ref()) + size(h.right.as_ref()) + 1;
    //     });
    //     x
    // }

    fn rotate_right(h: Box<Node<K, V>>) -> Box<Node<K, V>> {
        panic!("Not yet implemented");
    }

    fn flip_colors(h: Box<Node<K, V>>) -> Box<Node<K, V>> {
        panic!("Not yet implemented");
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
        let t = RedBlackTree::<String, i32>::new();

        assert!(!t.contains("MyStr".to_string()));
        assert_eq!(t.get("MyStr".to_string()), None);

        //TODO add an item and check the get and contains changes
    }
}
