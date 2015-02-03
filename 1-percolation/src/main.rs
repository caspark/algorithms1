#![feature(rand)] // so we can use random numbers without warnings
#![feature(core)] // otherwise we get a warning from generated code of #[derive(Debug)]

extern crate quickcheck;
use std::iter;

trait SafeToUsize {
    fn to_usize(&self) -> usize;
}

impl SafeToUsize for u32 {
    fn to_usize(&self) -> usize {
        *self as usize // Rust doesn't support < 32 bit pointers
    }
}

pub trait UnionFind {
    fn union(&mut self, p: u32, q: u32);
    fn connected(&self, p: u32, q: u32) -> bool;
}

#[derive(Debug)]
pub struct QuickUnionUF {
    id: Vec<u32>,
}

impl QuickUnionUF {
    pub fn new(size: u32) -> QuickUnionUF {
        QuickUnionUF { id: (0u32..size).collect() }
    }

    fn root(&self, mut i: u32) -> u32 {
        while i != self.id[i.to_usize()] {
            i = self.id[i.to_usize()];
        }
        i
    }
}

impl UnionFind for QuickUnionUF {
    fn union(&mut self, p: u32, q: u32) {
        let i = self.root(p);
        let j = self.root(q);
        self.id[i.to_usize()] = j;
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
            sz: iter::repeat(1u32).take(size as usize).collect(),
        }
    }

    fn root(&self, mut i: u32) -> u32 {
        while i != self.id[i.to_usize()] {
            i = self.id[i.to_usize()];
        }
        i
    }
}

impl UnionFind for WeightedQuickUnionUF {
    fn union(&mut self, p: u32, q: u32) {
        let i = self.root(p);
        let j = self.root(q);
        if i != j {
            if self.sz[i.to_usize()] < self.sz[j.to_usize()] {
                self.id[i.to_usize()] = j;
                self.sz[j.to_usize()] += self.sz[i.to_usize()];
            } else {
                self.id[j.to_usize()] = i;
                self.sz[i.to_usize()] += self.sz[j.to_usize()];
            }
        }
    }

    fn connected(&self, p: u32, q: u32) -> bool {
        self.root(p) == self.root(q)
    }
}

mod tests {
    use std::rand;
    use std::rand::Rng;
    use quickcheck::{quickcheck, QuickCheck, StdGen};
    use SafeToUsize;
    use super::UnionFind;
    use super::QuickUnionUF;

    #[test]
    fn test_each_number_is_own_root() {
        fn each_number_is_own_root(size: u32) -> bool {
            let qu = QuickUnionUF::new(size);
            for i in 0..size {
                if i != qu.root(i) {
                    return false;
                }
            }
            true
        }
        quickcheck(each_number_is_own_root as fn(u32) -> bool);
    }

    #[test]
    fn test_nothing_is_connected_without_any_unions() {
        fn nothing_is_connected_without_any_unions(size: u32) -> bool {
            let qu = QuickUnionUF::new(size);
            for i in 0..size {
                for j in i..size {
                    if i != j && qu.connected(i, j) {
                        return false;
                    }
                }
            }
            true
        }
        quickcheck(nothing_is_connected_without_any_unions as fn(u32) -> bool);
    }

    #[test]
    fn test_connecting_nodes_works() {
        fn connecting_nodes_works(sizes: Vec<u32>) -> bool {
            use std::cmp;

            // set some constraints to avoid having this property take forever
            let max_size_per_group = 1001;
            let max_node_count = 5000;

            let mut node_count = 0u32;
            let mut node_groups: Vec<Vec<u32>> = Vec::with_capacity(sizes.len());
            for &size in sizes.iter() {
                let limited_size = cmp::min(size % max_size_per_group, max_node_count);
                if limited_size > 0 {
                    let nodes = (node_count .. (node_count + limited_size)).collect::<Vec<u32>>();
                    node_count += limited_size;
                    node_groups.push(nodes);
                }
            }
            // println!("node_groups has {} nodes: {:?}", node_count, node_groups);

            let nodes_to_union: Vec<(u32, u32)> = {
                let mut rng = rand::thread_rng(); // TODO use http://doc.rust-lang.org/std/rand/trait.SeedableRng.html
                let mut unions = Vec::with_capacity(node_count.to_usize());
                for nodes in node_groups.iter() {
                    let mut shuffled_nodes = nodes.clone();
                    rng.shuffle(shuffled_nodes.as_mut_slice());
                    for window in shuffled_nodes[].windows(2) {
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

            let mut qu = QuickUnionUF::new(node_count);
            for &(p, q) in nodes_to_union.iter() {
                // println!("Union: {}, {}", p, q);
                qu.union(p, q);
            }

            matches_connection_state(&qu, &node_groups)
        }
        assert!(connecting_nodes_works(vec![0, 2, 4, 8, 15]), "the example failed");
        QuickCheck::new().gen(StdGen::new(rand::thread_rng(), 25)) // generate vecs with max size 25
            .quickcheck(connecting_nodes_works as fn(Vec<u32>) -> bool);
    }

    fn matches_connection_state(qu: &QuickUnionUF, node_groups: &Vec<Vec<u32>>) -> bool {
        use std::collections::HashMap;

        let mut expected_node_groups = HashMap::<u32, u32>::new();
        let mut all_nodes = Vec::new();
        for (group_num, nodes) in node_groups.iter().enumerate() {
            for node in nodes.iter() {
                expected_node_groups.insert(*node, group_num as u32); // FIXME this case could fail silently
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
