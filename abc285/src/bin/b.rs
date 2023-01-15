use proconio::{input, fastout};
use proconio::marker::Chars;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!{
        n: usize,
        s: Chars,
        }

    for i in 1..n {
        let mut l = 0;
        while s[l] != s[l+i] {
            if (l + i < n - 1) {
                l += 1;
            } else {
                l += 1;
                break;
            }
        }
        println!("{}", l);
    }

}
