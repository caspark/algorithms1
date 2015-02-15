use std::option::Option;
use std::iter::Iterator;
use std::{mem, ptr, fmt};

struct Node<E> {
    item: E,
    next: Option<Box<Node<E>>>,
    prev: Rawlink<Node<E>>,
}

// copied from Rust std's DList's Rawlink
struct Rawlink<E> {
    p: *mut E,
}

// copied from Rust std's DList's Rawlink; this is like Option but for a raw pointer
impl<T> Rawlink<T> {
    #![allow(dead_code)]

    /// Like Option::None for Rawlink
    fn none() -> Rawlink<T> {
        Rawlink{p: ptr::null_mut()}
    }

    /// Like Option::Some for Rawlink
    fn some(n: &mut T) -> Rawlink<T> {
        Rawlink{p: n}
    }

    /// Convert the `Rawlink` into an Option value
    fn resolve_immut<'a>(&self) -> Option<&'a T> {
        unsafe {
            mem::transmute(self.p.as_ref())
        }
    }

    /// Convert the `Rawlink` into an Option value
    fn resolve<'a>(&mut self) -> Option<&'a mut T> {
        if self.p.is_null() {
            None
        } else {
            Some(unsafe { mem::transmute(self.p) })
        }
    }

    /// Return the `Rawlink` and replace with `Rawlink::none()`
    fn take(&mut self) -> Rawlink<T> {
        mem::replace(self, Rawlink::none())
    }
}

pub struct Iter<'a, E: 'a> {
    head: &'a Option<Box<Node<E>>>,
    nelem: usize,
}

pub struct Deque<E> {
    size: usize,
    first: Option<Box<Node<E>>>,
}

impl<E> Deque<E> {
    pub fn new() -> Deque<E> {
        Deque {
            first: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn add_first(&mut self, item: E) {
        self.size += 1;
        let mut boxed_new_head = Box::new(Node {
            item: item,
            next: None,
            prev: Rawlink::none(),
        });
        match self.first {
            None => self.first = Some(boxed_new_head),
            Some(ref mut head) => {
                head.prev = Rawlink::some(&mut *boxed_new_head);
                mem::swap(head, &mut boxed_new_head);
                head.next = Some(boxed_new_head);
            },
        };
    }

    pub fn remove_first(&mut self) -> Option<E> {
        self.first.take().map(|mut boxed_first_node| {
            self.size -= 1;
            match boxed_first_node.next.take() {
                None => {} //nothing to do here
                Some(mut node) => {
                    node.prev = Rawlink::none();
                    self.first = Some(node);
                }
            }
            boxed_first_node.item
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

impl<A: fmt::Debug> fmt::Debug for Deque<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Deque ["));

        for (i, e) in self.iter().enumerate() {
            if i != 0 { try!(write!(f, ", ")); }
            try!(write!(f, "{:?}", *e));
        }

        write!(f, "]")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_should_increase_size() {
        let mut sut = Deque::<u32>::new();
        sut.add_first(0);
        sut.add_first(1);
        assert_eq!(sut.len(), 2);
    }

    #[test]
    fn removing_should_get_previous_item_added() {
        let mut sut = Deque::<u32>::new();
        sut.add_first(0);
        sut.add_first(1);
        assert_eq!(sut.remove_first(), Some(1));
        assert_eq!(sut.remove_first(), Some(0));
    }

    #[test]
    fn removing_from_empty_should_get_none() {
        let mut sut = Deque::<u32>::new();
        assert_eq!(sut.remove_first(), None);
    }

    #[test]
    fn removing_should_decrease_size() {
        let mut sut = Deque::<u32>::new();
        sut.add_first(1);
        sut.remove_first();
        assert_eq!(sut.len(), 0);
    }

    //TODO write and test backwards iteration

    #[test]
    fn iteration_should_work() {
        let mut sut = Deque::<usize>::new();
        sut.add_first(1);
        sut.add_first(2);
        sut.add_first(3);
        for (i, &e) in sut.iter().enumerate() {
            assert_eq!(sut.len() - i, e);
        }
    }

    #[test]
    fn removing_first_should_prev_link_on_new_first_node() {
        let mut sut = Deque::new();
        sut.add_first(0);
        sut.add_first(1);
        {
            sut.remove_first();
        }
        let maybe_prev_node = (*sut.first.unwrap()).prev.resolve_immut();
        assert!(maybe_prev_node.is_none(), "New first node should not be pointing to removed first node");
    }

    #[test]
    fn test_show() {
        let mut sut = Deque::new();
        sut.add_first(3);
        sut.add_first(2);
        sut.add_first(1);
        assert_eq!(format!("{:?}", sut), "Deque [1, 2, 3]");

        let mut sut = Deque::new();
        sut.add_first("test");
        sut.add_first("more");
        sut.add_first("one");
        sut.add_first("just");
        assert_eq!(format!("{:?}", sut), "Deque [\"just\", \"one\", \"more\", \"test\"]");
    }
}
