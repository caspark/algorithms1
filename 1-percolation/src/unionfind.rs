use std::iter;
use conversions::AsUsizeConverter;

pub trait UnionFind {
    fn union(&mut self, p: u32, q: u32);
    fn connected(&self, p: u32, q: u32) -> bool;
}

#[derive(Debug)]
pub struct QuickUnionUF {
    id: Vec<u32>,
}

impl QuickUnionUF {
    #![allow(dead_code)]
    pub fn new(size: u32) -> QuickUnionUF {
        QuickUnionUF { id: (0u32..size).collect() }
    }

    fn root(&self, mut i: u32) -> u32 {
        while i != self.id[i.as_usize()] {
            i = self.id[i.as_usize()];
        }
        i
    }
}

impl UnionFind for QuickUnionUF {
    fn union(&mut self, p: u32, q: u32) {
        let i = self.root(p);
        let j = self.root(q);
        self.id[i.as_usize()] = j;
    }

    fn connected(&self, p: u32, q: u32) -> bool {
        self.root(p) == self.root(q)
    }
}

#[derive(Debug)]
pub struct WeightedQuickUnionUF {
    id: Vec<u32>,
    sz: Vec<u32>,
}

impl WeightedQuickUnionUF {
    pub fn new(size: u32) -> WeightedQuickUnionUF {
        WeightedQuickUnionUF {
            id: (0u32..size).collect(),
            sz: iter::repeat(1u32).take(size.as_usize()).collect(),
        }
    }

    fn root(&self, mut i: u32) -> u32 {
        while i != self.id[i.as_usize()] {
            i = self.id[i.as_usize()];
        }
        i
    }
}

impl UnionFind for WeightedQuickUnionUF {
    fn union(&mut self, p: u32, q: u32) {
        let i = self.root(p);
        let j = self.root(q);
        if i != j {
            if self.sz[i.as_usize()] < self.sz[j.as_usize()] {
                self.id[i.as_usize()] = j;
                self.sz[j.as_usize()] += self.sz[i.as_usize()];
            } else {
                self.id[j.as_usize()] = i;
                self.sz[i.as_usize()] += self.sz[j.as_usize()];
            }
        }
    }

    fn connected(&self, p: u32, q: u32) -> bool {
        self.root(p) == self.root(q)
    }
}

#[cfg(test)]
mod tests {
    use rand;
    use rand::Rng;
    use quickcheck::{StdGen, QuickCheck};
    use super::super::conversions::{AsUsizeConverter, TryU32Converter};
    use super::UnionFind;
    use super::{QuickUnionUF, WeightedQuickUnionUF};

    #[test]
    fn quickunion_connecting_nodes_works() {
        fn connecting_nodes_works(sizes: Vec<u32>) -> bool {
            let (node_count, nodes_to_union, expected_groups) = generate_unions(&sizes);

            let mut qu = QuickUnionUF::new(node_count);
            if !matches_connection_state(&qu, &(0u32 .. node_count).map(|node| vec![node]).collect()) {
                return false;
            }

            for &(p, q) in nodes_to_union.iter() {
                // println!("Union: {}, {}", p, q);
                qu.union(p, q);
            }
            matches_connection_state(&qu, &expected_groups)
        }
        QuickCheck::new().gen(StdGen::new(rand::thread_rng(), 25)) // generate vecs with max size 25
            .quickcheck(connecting_nodes_works as fn(Vec<u32>) -> bool);
    }


    #[test]
    fn weighted_quickunion_connecting_nodes_works() {
        fn connecting_nodes_works(sizes: Vec<u32>) -> bool {
            let (node_count, nodes_to_union, expected_groups) = generate_unions(&sizes);

            let mut qu = WeightedQuickUnionUF::new(node_count);
            if !matches_connection_state(&qu, &(0u32 .. node_count).map(|node| vec![node]).collect()) {
                return false;
            }

            for &(p, q) in nodes_to_union.iter() {
                // println!("Union: {}, {}", p, q);
                qu.union(p, q);
            }
            matches_connection_state(&qu, &expected_groups)
        }
        QuickCheck::new().gen(StdGen::new(rand::thread_rng(), 25)) // generate vecs with max size 25
            .quickcheck(connecting_nodes_works as fn(Vec<u32>) -> bool);
    }

    /// Given a list of group sizes, returns the number of nodes, the unions to make, and the final expected groups.
    /// Current implementation limitations:
    /// - the unions & their ordering is currently non-deterministic
    /// - the final connected groups will consist of consequtive numbers
    fn generate_unions(sizes: &Vec<u32>) -> (u32, Vec<(u32, u32)>, Vec<Vec<u32>>) {
        use std::cmp;

        // set some constraints to avoid having this property take forever
        let max_size_per_group = 1001;
        let max_node_count = 5000;

        let mut node_count = 0u32;
        let mut expected_groups: Vec<Vec<u32>> = Vec::with_capacity(sizes.len());
        for &size in sizes.iter() {
            let limited_size = cmp::min(size % max_size_per_group, max_node_count);
            if limited_size > 0 {
                let nodes = (node_count .. (node_count + limited_size)).collect::<Vec<u32>>();
                node_count += limited_size;
                expected_groups.push(nodes);
            }
        }
        // println!("expected_groups has {} nodes: {:?}", node_count, expected_groups);

        let nodes_to_union: Vec<(u32, u32)> = {
            let mut rng = rand::thread_rng(); // TODO use http://doc.rust-lang.org/std/rand/trait.SeedableRng.html
            let mut unions = Vec::with_capacity(node_count.as_usize());
            for nodes in expected_groups.iter() {
                let mut shuffled_nodes = nodes.clone();
                rng.shuffle(shuffled_nodes.as_mut_slice());
                for window in shuffled_nodes[].windows(2) {
                    // wtf, there has to be a better way to convert a vector of 2 elements into a tuple :(
                    let window_nodes = window.iter().map(|&a| a).collect::<Vec<u32>>();
                    match &window_nodes[] {
                        [p, q] => unions.push((p, q)),
                        _ => unreachable!()
                    }
                }
            }
            rng.shuffle(unions.as_mut_slice());
            unions
        };

        (node_count, nodes_to_union, expected_groups)
    }

    fn matches_connection_state(qu: &UnionFind, node_groups: &Vec<Vec<u32>>) -> bool {
        use std::collections::HashMap;

        let mut expected_node_groups = HashMap::<u32, u32>::new();
        let mut all_nodes = Vec::new();
        for (group_num, nodes) in node_groups.iter().enumerate() {
            for node in nodes.iter() {
                expected_node_groups.insert(*node, group_num.try_u32());
                all_nodes.push(node);
            }
        };

        for &p in all_nodes.iter() {
            let p_group = expected_node_groups.get(p).unwrap();
            for &q in all_nodes.iter() {
                let expect_connected = p_group == expected_node_groups.get(q).unwrap();
                if p != q && (qu.connected(*p, *q) != expect_connected || qu.connected(*q, *p) != expect_connected) {
                    println!("{} and {} are not connected", p, q);
                    return false;
                }
            }
        }
        true
    }

}
