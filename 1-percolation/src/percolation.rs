use std::iter;
use conversions::{AsUsizeConverter, TryU32Converter};
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
    use rand::distributions::range::Range;
    use rand::distributions::IndependentSample;
    use rand::ThreadRng;
    use rand;

    fn pick_closed_site(perc: &Percolation, range: &mut Range<usize>, rng: &mut ThreadRng) -> (usize, usize) {
        loop {
            let (i, j) = ((*range).ind_sample(rng), range.ind_sample(rng));
            if !perc.is_open(i, j) {
                return (i, j);
            }
        }
    }

    let mut rng = rand::thread_rng();
    let mut random_range = Range::new(1, n + 1);
    let mut perc = Percolation::new(n);
    let mut opened = 0u32;
    while !perc.percolates() {
        let (i, j) = pick_closed_site(&perc, &mut random_range, &mut rng);
        perc.open(i, j);
        opened += 1;
    }
    opened as f32 / (n * n) as f32
}

pub fn simulate_multiple(n: usize, times: usize) -> PercolationStats {
    let mut results = Vec::with_capacity(times as usize);
    for _ in 0 .. times {
        results.push(simulate(n));
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
