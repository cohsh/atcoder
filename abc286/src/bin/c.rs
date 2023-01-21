use proconio::{input, fastout};
use proconio::marker::Chars;
use std::cmp;

#[fastout]
fn main() {
    input!{
        n: usize, a: usize, b: usize,
        mut s: Chars,
    }

    let mid: usize;
    if n % 2 == 0 {
        mid = n / 2;
    } else {
        mid = (n - 1) / 2;
    }

    let mut answer = 0;
    for i in 0..mid {
        if s[i] != s[n-i-1] {
            answer += b;
        }
    }

    for c in 1..n {
        s.rotate_left(1);
        let mut cost = 0;
        cost += a * c;
        for i in 0..mid {
            if s[i] != s[n-i-1] {
                cost += b;
            }
        }
        answer = cmp::min(answer, cost);
    }

    println!("{}", answer);
}
