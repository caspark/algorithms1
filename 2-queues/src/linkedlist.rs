use std::option::Option;
use std::iter::Iterator;

#[derive(Debug)]
struct Node<E> {
    item: E,
    next: Option<Box<Node<E>>>,
}

pub struct Iter<'a, E: 'a> {
    head: &'a Option<Box<Node<E>>>,
    nelem: usize,
}

#[derive(Debug)]
pub struct LinkedList<E> {
    size: usize,
    first: Option<Box<Node<E>>>,
}

impl<E> LinkedList<E> {
    pub fn new() -> LinkedList<E> {
        LinkedList {
            first: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn add_first(&mut self, item: E) {
        self.first = Some(Box::new(Node {
            item: item,
            next: self.first.take(), // take is necessary to take ownership of the item in the option
        }));
        self.size += 1;
    }

    pub fn remove_first(&mut self) -> Option<E> {
        self.size -= 1;
        self.first.take().map(|boxed_node| {
            let node = *boxed_node;
            self.first = node.next; // mutating state in a map, wooo!
            node.item
        })
    }

    pub fn iter(&self) -> Iter<E> {
        Iter {
            nelem: self.len(),
            head: &self.first,
        }
    }
}

impl<'a, A> Iterator for Iter<'a, A> {
    type Item = &'a A;

    fn next(&mut self) -> Option<&'a A> {
        if self.nelem == 0 {
            return None;
        }
        self.head.as_ref().map(|head| {
            self.nelem -= 1;
            self.head = &head.next;
            &head.item
        })
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

    #[test]
    fn removing_should_get_previous_item_added() {
        let mut sut = LinkedList::<u32>::new();
        sut.add_first(0);
        sut.add_first(1);
        assert_eq!(sut.remove_first(), Some(1));
        assert_eq!(sut.remove_first(), Some(0));
    }

    #[test]
    fn removing_from_empty_should_get_none() {
        let mut sut = LinkedList::<u32>::new();
        assert_eq!(sut.remove_first(), None);
    }

    #[test]
    fn removing_should_decrease_size() {
        let mut sut = LinkedList::<u32>::new();
        sut.add_first(1);
        sut.remove_first();
        assert_eq!(sut.len(), 0);
    }

    #[test]
    fn iteration_should_work() {
        let mut sut = LinkedList::<usize>::new();
        sut.add_first(1);
        sut.add_first(2);
        sut.add_first(3);
        for (i, &e) in sut.iter().enumerate() {
            assert_eq!(sut.len() - i, e);
        }
    }
}
