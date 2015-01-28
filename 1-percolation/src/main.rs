#[derive(Debug)]
struct QuickUnionUF {
    id: Vec<i32>,
}

impl QuickUnionUF {
    fn new(size: i32) -> QuickUnionUF {
        let mut v = Vec::with_capacity(size as usize);
        for i in 0..size {
            v.push(i);
        }
        QuickUnionUF { id: v }
    }

    fn union(&mut self, p: i32, q: i32) {
        //TODO
    }

    fn connected(&self, p: i32, q: i32) -> bool {
        self.root(p) == self.root(q)
    }

    fn find(&self, p: i32, q: i32) {
        //TODO
    }

    fn count() {

    }

    fn root(&self, mut i: i32) -> i32 {
        while i != self.id[i as usize] {
            i = self.id[i as usize];
        }
        i
    }
}

fn main() {
    println!("Percolation");

    let qu = QuickUnionUF::new(10);
    println!("Got a {:?}", qu);

    println!("Root of element #1 is {}", qu.root(1));
}
