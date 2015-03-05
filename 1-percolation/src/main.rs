#![feature(core)] // needed for a whole bunch of unstable features
#![feature(collections)] // so we can get the tail of a vector
#![feature(test)] // so we can run benchmarks

extern crate getopts;
#[cfg(test)]
extern crate quickcheck;
extern crate rand;
#[cfg(test)]
extern crate test;

mod conversions;
mod unionfind;
mod percolation;
#[cfg(test)]
mod benchmarks;

fn main() {
    use std::env;
    use getopts::Options;

    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.reqopt("n", "size", "Size of each side of the percolation board", "SIZE");
    opts.optopt("t", "times", "Number of percolations to simulate", "TIMES");
    opts.optopt("j", "jobs", "Maximum number of jobs (threads) to use", "JOBS");
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
                let jobs_str = matches.opt_str("j").unwrap_or("1".to_string());

                let parse_result = size_str.parse::<usize>().and_then(|size| {
                    times_str.parse::<usize>().and_then(|times| {
                        jobs_str.parse::<u32>().map(|jobs| (size, times, jobs))
                    })
                });
                match parse_result {
                    Ok((size, times, jobs)) => {
                        use percolation;

                        println!("Running {num} percolation(s) on a {n}x{n} board using max {jobs} job(s)",
                            num=times, n=size, jobs=jobs);
                        let stats = percolation::simulate_multiple(size, times, jobs);
                        // println!("{:?}", stats);
                        println!("Mean: {}", stats.mean());
                    },
                    Err(_) => println!("Failed to convert arguments of -t,  -n, or -j to numbers (try --help)")
                };
            }
        },
        Err(f) => println!("{}\nUse --help for usage information", f.to_string()),
    }
}
