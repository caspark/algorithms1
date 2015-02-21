use rand::{self, Rng};
use std::iter::Iterator;

#[derive(Debug)]
pub struct RandomQueue<E> {
    vec: Vec<E>, // our backing "array". Vectors resize, but for the purpose of this exercise we'll assume they can't
}


#[derive(Debug)]
pub struct Iter<E> {
    vec: Vec<E>,
}

impl <E: Clone> RandomQueue<E> {
    pub fn new() -> RandomQueue<E> {
        RandomQueue {
            vec: Vec::with_capacity(2)
        }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn enqueue(&mut self, item: E) {
        if self.vec.len() == self.vec.capacity() {
            // unnecessary resize time!
            let mut new_vec = Vec::with_capacity(self.vec.capacity() * 2);
            new_vec.push_all(&self.vec);
            self.vec = new_vec;
        }
        self.vec.push(item);
    }

    pub fn dequeue(&mut self) -> Option<E> {
        if self.vec.len() == 0 {
            return None;
        }
        if self.vec.len() == self.vec.capacity() / 4 {
            // unnecessary resize time!
            let mut new_vec = Vec::with_capacity(self.vec.capacity() / 2);
            new_vec.push_all(&self.vec);
            self.vec = new_vec;
        }
        let curr_len = self.vec.len();
        let picked = rand::thread_rng().gen_range(0, curr_len);
        self.vec.as_mut_slice().swap(picked, curr_len - 1);
        self.vec.pop()
    }

    pub fn sample(&self) -> Option<E> {
        if self.len() == 0 {
            None
        } else {
            let picked = rand::thread_rng().gen_range(0, self.vec.len());
            Some(self.vec[picked].clone())
        }
    }

    pub fn iter(&self) -> Iter<E> {
        let mut items = self.vec.clone();
        let mut rng = rand::thread_rng();
        for i in 0 .. items.len() {
            let r = rng.gen_range(0, items.len());
            items.as_mut_slice().swap(i, r);
        }
        Iter {
            vec: items,
        }
    }
}

impl<E> Iterator for Iter<E> {
    type Item = E;

    fn next(&mut self) -> Option<E> {
        self.vec.pop()
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

    #[test]
    fn iteration_includes_all_items_once() {
        let mut sut = RandomQueue::new();
        let original = vec![1,2,3,4];
        for &i in original.iter() {
            sut.enqueue(i.clone());
        }

        let mut randomized = sut.iter().collect::<Vec<u32>>();
        randomized.as_mut_slice().sort();

        assert_eq!(randomized, original);
    }
}
