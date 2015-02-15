use rand::{self, Rng};
use std::iter::Iterator;

#[derive(Debug)]
pub struct RandomQueue<E> {
    vec: Vec<E>, // our backing "array". Vectors resize, but for the purpose of this exercise we'll assume they can't
}


#[derive(Debug)]
pub struct Iter<'a, E: 'a> {
    vec: Vec<E>,
    nelem: usize,
}

impl <E: Clone> RandomQueue<E> {
    fn new() -> RandomQueue<E> {
        RandomQueue {
            vec: Vec::with_capacity(2)
        }
    }

    fn len(&self) -> usize {
        self.vec.len()
    }

    fn enqueue(&mut self, item: E) {
        if self.vec.len() == self.vec.capacity() {
            // unnecessary resize time!
            let mut new_vec = Vec::with_capacity(self.vec.capacity() * 2);
            new_vec.push_all(&self.vec[]);
            self.vec = new_vec;
        }
        self.vec.push(item);
    }

    fn dequeue(&mut self) -> Option<E> {
        if self.vec.len() == self.vec.capacity() / 4 {
            // unnecessary resize time!
            let mut new_vec = Vec::with_capacity(self.vec.capacity() / 2);
            new_vec.push_all(&self.vec[]);
            self.vec = new_vec;
        }
        self.vec.pop() //FIXME make this random
    }

    fn sample(&self) -> Option<E> {
        if self.len() == 0 {
            None
        } else {
            Some(self.vec[self.vec.len() - 1].clone()) //FIXME make this random
        }
    }

    fn iter(&self) -> Iter<E> {
        panic!("Not yet implemented");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_is_reported_properly() {
        let mut sut = RandomQueue::new();
        assert_eq!(sut.len(), 0);
        sut.enqueue(0);
        sut.enqueue(1);
        assert_eq!(sut.len(), 2);
        sut.sample();
        assert_eq!(sut.len(), 2);
        sut.dequeue();
        assert_eq!(sut.len(), 1);
    }

    #[test]
    fn sample_and_deque_pick_from_elements_in_array() {
        let mut sut = RandomQueue::new();
        sut.enqueue(1);
        assert_eq!(sut.sample().unwrap(), 1);
        assert_eq!(sut.dequeue().unwrap(), 1);
    }

    #[test]
    fn sample_and_dequeue_of_empty_queue_is_none() {
        let mut sut = RandomQueue::new();
        assert_eq!(sut.dequeue(), None);
        assert_eq!(sut.sample(), None);
        sut.enqueue(1);
        sut.dequeue();
        assert_eq!(sut.dequeue(), None);
        assert_eq!(sut.sample(), None);
    }

    #[test]
    fn resizes_when_too_many_or_too_few_elements() {
        let mut sut = RandomQueue::new();
        assert_eq!(sut.vec.capacity(), 2);
        sut.enqueue(1);
        sut.enqueue(2);
        assert_eq!(sut.vec.capacity(), 2);
        sut.enqueue(3);
        assert_eq!(sut.vec.capacity(), 4);
        sut.enqueue(4);
        sut.enqueue(5);
        assert_eq!(sut.vec.capacity(), 8);

        sut.dequeue(); // 4 after dequeue
        sut.dequeue(); // 3 after dequeue
        sut.dequeue(); // 2 after dequeue
        assert_eq!(sut.vec.capacity(), 8);
        sut.dequeue(); // 1 after dequeue
        assert_eq!(sut.vec.capacity(), 4);
    }
}
