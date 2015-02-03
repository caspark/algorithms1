extern crate quickcheck;

trait SafeToUsize {
    fn to_usize(&self) -> usize;
}

impl SafeToUsize for u32 {
    fn to_usize(&self) -> usize {
        *self as usize // Rust doesn't support < 32 bit pointers
    }
}

mod quickunion {
    use SafeToUsize;

    #[derive(Debug)]
    pub struct QuickUnionUF {
        id: Vec<u32>,
    }

    impl QuickUnionUF {
        pub fn new(size: u32) -> QuickUnionUF {
            QuickUnionUF { id: (0u32..size).collect() }
        }

        pub fn union(&mut self, p: u32, q: u32) {
            let i = self.root(p);
            let j = self.root(q);
            self.id[i.to_usize()] = j;
        }

        pub fn connected(&self, p: u32, q: u32) -> bool {
            self.root(p) == self.root(q)
        }

        pub fn root(&self, mut i: u32) -> u32 {
            while i != self.id[i.to_usize()] {
                i = self.id[i.to_usize()];
            }
            i
        }
    }

    mod tests {
        use quickcheck::quickcheck;
        use SafeToUsize;
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
                use std::rand::{thread_rng, Rng};

                if sizes.len() == 0 {
                    return true;
                }

                let mut node_count = 0u32;
                let mut node_groups: Vec<Vec<u32>> = Vec::with_capacity(sizes.len());
                for &size in sizes.iter() {
                    let nodes = (node_count .. (node_count + size)).collect::<Vec<u32>>();
                    node_count += size;
                    node_groups.push(nodes);
                }
                println!("node_groups has {} nodes: {:?}", node_count, node_groups);

                let mut rng = thread_rng(); // TODO use http://doc.rust-lang.org/std/rand/trait.SeedableRng.html

                let nodes_to_union: Vec<(u32, u32)> = {
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
                    println!("Union: {}, {}", p, q);
                    qu.union(p, q);
                }

                for nodes in node_groups.iter() {
                    if !are_connected(&qu, nodes) {
                        return false
                    }
                }
                true
            }
            // assert!(connecting_nodes_works(vec![0, 2, 4, 8]), "the example failed");
            quickcheck(connecting_nodes_works as fn(Vec<u32>) -> bool);
        }

        fn are_connected(qu: &QuickUnionUF, nodes: &Vec<u32>) -> bool {
            for &p in nodes.iter() {
                for &q in nodes.iter() {
                    if p != q && (!qu.connected(p, q) || !qu.connected(q, p)) {
                        println!("{} and {} are not connected", p, q);
                        return false;
                    }
                }
            }
            true
        }

    }
}

#[allow(dead_code)]
fn main() {
    use quickunion::QuickUnionUF;

    println!("Percolation");

    let mut qu = QuickUnionUF::new(10);
    println!("Got a {:?}", qu);

    println!("Root of element #1 is {}", qu.root(1));

    assert_eq!(qu.root(1), 1);
    qu.union(1, 2);
    assert_eq!(qu.root(1), 2);
    assert_eq!(qu.root(2), 2);
    assert!(qu.connected(1,2), "1 and 2 should be connected");
}
