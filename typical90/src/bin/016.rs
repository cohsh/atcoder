use proconio::{input, fastout};
use std::cmp;

const M: isize = 9999;

#[fastout]
fn main() {
    input!{n: isize, a: isize, b: isize, c: isize}
    let mut answer = 1 << 30;
    for i in 0..=M {
        for j in 0..=M-i {
            let v = n - a * i - b * j;
            if v % c == 0 {
                let k = v / c;
                if k >= 0 && k <= M {
                    answer = cmp::min(answer, i + j + k);
                }
            }
        }
    }
    println!("{}", answer);
}
