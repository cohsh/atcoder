use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!{
        n: usize,
        c: [[usize; 2]; n],
    }

    let mut candidates = vec![vec![0; 0]; n+1];

    for i in 0..n {
        candidates[c[i][0]].push(c[i][1]);
        candidates[c[i][1]].push(c[i][0]);
    }

    let mut j: usize;
    let mut answer = 1;
    for i in 1..=n {
        if answer
    }

    println!("{}", answer);
}
