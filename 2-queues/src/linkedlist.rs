use std::option::Option;

#[derive(Debug)]
struct Node<E> {
    item: E,
    next: Option<Box<Node<E>>>,
}

#[derive(Debug)]
pub struct LinkedList<E> {
    size: u32,
    first: Option<Box<Node<E>>>,
}

impl<E> LinkedList<E> {
    pub fn new() -> LinkedList<E> {
        LinkedList {
            first: None,
            size: 0,
        }
    }

    pub fn len(&self) -> u32 {
        self.size
    }

    pub fn add_first(&mut self, item: E) {
        self.first = Some(Box::new(Node {
            item: item,
            next: self.first.take(), // take is necessary to take ownership of the item in the option
        }));
        self.size += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_should_increase_size() {
        let mut sut = LinkedList::<u32>::new();
        sut.add_first(0);
        sut.add_first(1);
        assert_eq!(sut.len(), 2);
    }
}
