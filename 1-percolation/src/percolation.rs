use std::iter;
use conversions::AsUsizeConverter;
use std::num::Int;

pub struct Percolation {
    n: usize,
    grid: Vec<bool>,
}

impl Percolation {
    pub fn new(n: usize) -> Percolation {
        Percolation {
            n: n,
            grid: iter::repeat(false).take(n.pow(2)).collect(),
        }
    }

    fn to_index(&self, i: usize, j: usize) -> Result<usize, Box<String>> {
        if i < 1 {
            Err(Box::new(format!("Out of bounds: i < 1: {} < 1", i)))
        } else if i > self.n {
            Err(Box::new(format!("Out of bounds: i > self.n: {} > {}", i, self.n)))
        } else if j < 1 {
            Err(Box::new(format!("Out of bounds: j < 1: {} < 1", j)))
        } else if j > self.n {
            Err(Box::new(format!("Out of bounds: j > self.n: {} > {}", j, self.n)))
        } else {
            Ok((i - 1) + (j - 1) * self.n)
        }
    }

    pub fn open(&mut self, i: usize, j: usize) {
        // self.to_index(i, j).and_then(|index| {
        //
        // });
    }

    pub fn is_open(&self, i: usize, j: usize) -> bool {
        panic!("TODO")
    }

    pub fn is_full(&self, i: usize, j: usize) -> bool {
        panic!("TODO")
    }

    pub fn percolates(&self) -> bool {
        panic!("TODO")
    }
}
