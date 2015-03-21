struct Node<E> {
    next: Option<Box<Node<E>>>,
    val: Option<E>,
}

fn put_after_node<E>(maybe_node: &mut Option<Box<Node<E>>>, val: E) -> Node<E> {
    match maybe_node {
        &mut None => Node { next: None, val: Some(val) },
        &mut Some(ref mut node) => {
            let next = put_after_node(&mut node.next, val);
            Node { next: Some(Box::new(next)), val: node.val.take() }
        }
    }
}
