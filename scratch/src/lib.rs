struct Node {
    next: Option<Box<Node>>,
    val: i32,
}

fn put_after_node(maybe_node: Option<Box<Node>>, val: i32) -> Box<Node> {
    match maybe_node {
        None => Box::new(Node { next: None, val: val }),
        Some(mut node) => {
            node.next = Some(put_after_node(node.next.take(), val));
            node
        }
    }
}
