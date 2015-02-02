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
            let mut v = Vec::with_capacity(size.to_usize());
            for i in 0..size {
                v.push(i);
            }
            QuickUnionUF { id: v }
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

        #[test]
        fn test_each_number_is_own_root() {
            fn each_number_is_own_root(size: u32) -> bool {
                use quickunion::QuickUnionUF;
                let qu = QuickUnionUF::new(size as u32);
                for i in 0..(size as u32) {
                    if i != qu.root(i) {
                        return false;
                    }
                }
                true
            }
            quickcheck(each_number_is_own_root as fn(u32) -> bool);
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
