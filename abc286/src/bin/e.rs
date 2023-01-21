use proconio::{input, fastout};
use proconio::marker::Chars;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n],
        s: [Chars; n],
        q: usize,
        p: [[usize; 2]; q],
        }
    let mut answer = n;
    
    println!("{}", answer);
}
