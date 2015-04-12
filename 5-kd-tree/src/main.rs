#![feature(slice_patterns)] // slice pattern syntax is experimental ( Ok(Some([x1, y1, x2, y2])) => ... )

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate rand;

mod point;
mod rect;
mod redblacktree;
mod pointset;
