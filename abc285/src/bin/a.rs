use proconio::{input, fastout};
use proconio::marker::Chars;
use std::collections::VecDeque;
use std::cmp;

#[fastout]
fn main() {
    input!{
        a: usize, b: usize,
        }
    
    let c = cmp::min(a, b);
    let d = cmp::max(a, b);

    if (c * 2 == d) || (c * 2 + 1 == d) {
        println!("Yes");
    } else {
        println!("No");
    }
}
