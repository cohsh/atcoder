use proconio::{input, fastout};
use proconio::marker::Chars;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!{
        n: usize,
        s: [String; n],
        }
    let mut count = 0;

    for i in 0..n {
        if s[i] == "For" {
            count += 1;
        }
    }
    
    if count > n / 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
