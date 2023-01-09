use proconio::{input, fastout};
use std::cmp;

#[fastout]
fn main() {
    input!{n: usize, h: [usize; n]}

    let mut tmp = 0;
    let mut answer = 0;
    for i in 0..n {
        tmp = cmp::max(h[i], tmp);
        if tmp == h[i] {
            answer = i + 1;
        }
    }

    println!("{}", answer);
}
