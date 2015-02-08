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

    fn in_bounds(&self, i: usize, j: usize) -> bool {
        i >= 1 && i <= self.n && j >= 1 && j <= self.n
    }

    fn to_index(&self, i: usize, j: usize) -> usize {
        return (i - 1) + (j - 1) * self.n
    }

    fn assert_in_bounds(&self, i: usize, j: usize) {
        if !self.in_bounds(i, j) {
            panic!(format!("Out of bounds: ({i}, {j}) with n = {n}",
                    i=i, j=j, n=self.n))
        }
    }

    pub fn open(&mut self, i: usize, j: usize) {
        self.assert_in_bounds(i, j);
        let index = self.to_index(i, j);
        self.grid[index] = true;

        let neighbours = vec![(i, j - 1), (i, j + 1), (i + 1, j), (i - 1, j)];
        for (ni, nj) in neighbours {
            if self.in_bounds(ni, nj) {
                println!("Opening ({}, {})", ni, nj);
                let neighbour_index = self.to_index(ni, nj);
                self.grid[neighbour_index] = true;
            }
        }
    }

    pub fn is_open(&self, i: usize, j: usize) -> bool {
        self.assert_in_bounds(i, j);
        self.grid[self.to_index(i, j)]
    }

    pub fn percolates(&self) -> bool {
        panic!("TODO")
    }
}

mod tests{
    use super::Percolation;

    #[test]
    fn percolation_all_open_at_start() {
        let n = 10us;

        let perc = Percolation::new(n);
        for i in 1 .. n {
            for j in 1 .. n {
                assert!(!perc.is_open(i, j));
            }
        }
    }

    #[test]
    fn percolation_opening_works_properly() {
        let mut perc = Percolation::new(10us);

        perc.open(1, 1);
        assert!(perc.is_open(1, 1));
        assert!(perc.is_open(2, 1));
        assert!(perc.is_open(1, 2));
        assert!(!perc.is_open(2, 2));

        perc.open(10, 10);
        assert!(perc.is_open(10, 10));
        assert!(perc.is_open(9, 10));
        assert!(perc.is_open(10, 9));
        assert!(!perc.is_open(9, 9));

        for i in 3 .. 8us {
            for j in 3 .. 8us {
                assert!(!perc.is_open(i, j));
            }
        }
    }

    #[test]
    #[should_fail(expected = "Out of bounds: (9, 0)")]
    fn percolation_is_open_for_out_of_bounds_should_fail() {
        Percolation::new(10us).is_open(9, 0);
    }

    #[test]
    #[should_fail(expected = "Out of bounds: (0, 5)")]
    fn percolation_opening_out_of_bounds_should_fail() {
        Percolation::new(10us).open(0, 5);
    }
}
