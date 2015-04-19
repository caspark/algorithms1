#![feature(collections)] // for Vec.push_all()
#![feature(convert)] // as_mut_slice() is unstable, awaiting API revisions

extern crate rand;

mod deque;
mod linkedlist;
mod randomizedqueue;

/// Given a command line argument k and n space-separated strings on stdin, randomly print k of those n strings
fn main() {
    use std::env;

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error: missing numeric argument k: the number of items to subset from stdin");
        return;
    }
    match args[1].parse::<u32>() {
        Ok(k) => {
            use randomizedqueue::RandomQueue;
            use std::str;
            use std::io;
            use std::io::prelude::*;

            let stdin = io::stdin();
            let mut randomq = RandomQueue::new();
            for read_result in stdin.lock().split(' ' as u8) {
                match read_result {
                    //FIXME better to store strings here than bytes, but couldn't make the lifetimes work
                    Ok(read_bytes) => randomq.enqueue(read_bytes),
                    Err(err) => panic!("Error reading from stdin: {}", err),
                }
            }
            for _ in 0 .. k {
                let e = randomq.dequeue().expect("k must be <= number of strings provided to stdin");
                let e_as_str = str::from_utf8(&e).unwrap();
                println!("{}", e_as_str.trim());
            }
        },
        Err(_) => panic!("Error: received non-numeric k, where k is the number of items to subset from stdin"),
    }
}
