use proconio::{input, fastout};
use itertools::*;
use superslice::{self, Ext};

#[fastout]
fn main() {
    input!{n: usize, mut p: [usize; n]}
    p.prev_permutation();
    println!("{}", p.iter().join(" "));
}
