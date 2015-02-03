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
                use std::collections::HashMap;

                if sizes.len() == 0 {
                    return true;
                }

                let mut highest_node = 0u32;
                let mut groups_to_nodes = HashMap::new();
                for (group, &size) in sizes.iter().enumerate() {
                    let nodes = (highest_node .. (highest_node + size)).collect::<Vec<u32>>();
                    highest_node += size;
                    println!("highest is {}, nodes is {:?}", highest_node, nodes);
                    groups_to_nodes.insert(group, nodes);
                }

                println!("groups_to_nodes = {:?}", groups_to_nodes);

                let qu_size = highest_node + sizes[sizes.len() - 1];
                println!("Creating quickunion of size {}", qu_size);
                let mut qu = QuickUnionUF::new(qu_size);

                for nodes in groups_to_nodes.values() {
                    //TODO random union order
                    // use std::rand::{thread_rng, Rng};
                    // let mut rng = thread_rng(); // TODO use http://doc.rust-lang.org/std/rand/trait.SeedableRng.html
                    // let mut shuffled = nodes.clone();
                    // rng.shuffle(shuffled.as_mut_slice());
                    for window in nodes[].windows(2) {
                        let window_nodes = window.iter().map(|&a| a).collect::<Vec<u32>>();
                        match &window_nodes[] {
                            [p, q] => {
                                println!("Connecting {} and {}", p, q);
                                qu.union(p, q);
                            },
                            _ => unreachable!()
                        };
                    }
                }

                // TODO check that every group_num'th element is connected
                false
            }
            assert!(connecting_nodes_works(vec![2,4, 6]), "example failed");
            // quickcheck(connecting_nodes_works as fn(Vec<u32>) -> bool);
        }

    }
}

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
