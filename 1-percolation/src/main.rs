#![feature(core)] // otherwise we get a warning from generated code of #[derive(Debug)]

extern crate getopts;
extern crate quickcheck;
extern crate rand;

use getopts::Options;

mod conversions;
mod unionfind;
mod percolation;

fn main() {
    use getopts::Options;
    use std::os;

    let args: Vec<String> = os::args();

    let mut opts = Options::new();
    opts.reqopt("n", "size", "Size of each side of the percolation board", "SIZE");
    opts.optopt("t", "times", "Set number of percolation runs", "TIMES");
    opts.optflag("h", "help", "print this help menu");
    match opts.parse(args.tail()) {
        Ok(matches) => {
            if matches.opt_present("h") {
                let brief = format!("{}\n\nGathers percolation statistics according to the options provided.",
                        opts.short_usage(&args[0]));
                print!("{}", opts.usage(brief.as_slice()));
            } else {
                let size_str = matches.opt_str("n").expect("-n (or --size) should have been a required option");
                let times_str = matches.opt_str("t").unwrap_or("1".to_string());

                let parse_result = size_str.parse::<usize>().and_then(|size| {
                    times_str.parse::<usize>().map(|times| (size, times))
                });

                match parse_result {
                    Ok((size, times)) => {
                        use percolation;

                        println!("Running {num} percolation(s) on a {n}x{n} board", num=times, n=size);
                        let stats = percolation::simulate_multiple(size, times);
                        // println!("{:?}", stats);
                        println!("Mean: {}", stats.mean());
                    },
                    Err(_) => println!("Failed to convert arguments of -t (aka --times) and -n (--size) to numbers")
                };
            }
        },
        Err(f) => println!("{}\nUse --help for usage information", f.to_string()),
    }
}
