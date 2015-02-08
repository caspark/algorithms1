use std::iter;
use conversions::TryU32Converter;
use std::num::Int;
use unionfind::{UnionFind, WeightedQuickUnionUF};

pub struct Percolation {
    n: usize,
    grid: Vec<bool>,
    qu: WeightedQuickUnionUF,
}

impl Percolation {
    pub fn new(n: usize) -> Percolation {
        Percolation {
            n: n,
            grid: iter::repeat(false).take(n.pow(2)).collect(),
            qu: {
                let mut q = WeightedQuickUnionUF::new((n * n + 2).try_u32());
                for i in 1 .. (n + 1) {
                    q.union((i - 1).try_u32(), (n * n).try_u32());
                    q.union((n * (n - 1) + i - 1).try_u32(), (n * n + 1).try_u32());
                }
                q
            },
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
            if self.in_bounds(ni, nj) && self.is_open(ni, nj) {
                // println!("Connecting newly opened {:?} with already-open {:?}", (i, j), (ni, nj));
                let neighbour_index = self.to_index(ni, nj);
                self.qu.union(index.try_u32(), neighbour_index.try_u32());
            }
        }
    }

    pub fn is_open(&self, i: usize, j: usize) -> bool {
        self.assert_in_bounds(i, j);
        self.grid[self.to_index(i, j)]
    }

    pub fn percolates(&self) -> bool {
        self.qu.connected((self.n * self.n).try_u32(), (self.n * self.n + 1).try_u32())
    }
}

pub fn simulate(n: usize) -> f32 {
    use rand;
    use rand::Rng;

    let mut to_open: Vec<(usize, usize)> = Vec::with_capacity(n * n);
    for i in 1 .. (n + 1) {
        for j in 1 .. (n + 1) {
            to_open.push((i, j));
        }
    }
    rand::thread_rng().shuffle(to_open.as_mut_slice());

    let mut perc = Percolation::new(n);
    while !perc.percolates() {
        let (i, j) = to_open.pop().unwrap(); // safe: system must percolate before we run out of sites to open
        perc.open(i, j);
    }
    (n*n - to_open.len()) as f32 / (n * n) as f32
}

pub fn simulate_multiple(n: usize, times: usize, jobs: u32) -> PercolationStats {
    use std::thread::Thread;
    use std::sync::{Arc, Mutex, mpsc};

    // simulation runs left. Data doesn't matter - it's only used to distribute work across jobs.
    let sims_left = Arc::new(Mutex::new((0..times).collect::<Vec<usize>>()));

    let (tx, rx) = mpsc :: channel();
    for _ in 0 .. jobs {
        let tx = tx.clone();
        let sims_left = sims_left.clone();
        Thread::spawn(move|| {
            // acquire lock, fail if another task has failed, try to pop an item, and only continue if we got something
            while sims_left.lock().unwrap().pop().is_some() {
                tx.send(simulate(n)).unwrap();
            }
        });
    }
    let mut results = Vec::with_capacity(times);
    for _ in 0 .. times {
        let r = rx.recv().unwrap();
        results.push(r);
    }
    PercolationStats {
        results: results,
    }
}

#[derive(Debug)]
pub struct PercolationStats {
    results: Vec<f32>,
}

impl PercolationStats {
    pub fn mean(&self) -> f32 {
        let mut mean = 0f32;
        for (i, &r) in self.results.iter().enumerate() {
            mean = (mean * i as f32 + r ) / (i + 1) as f32;
        }
        mean
    }
}

#[cfg(test)]
mod tests {
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

        perc.open(10, 10);
        assert!(perc.is_open(10, 10));

        for i in 2 .. 9us {
            for j in 2 .. 9us {
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
