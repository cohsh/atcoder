use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{
        n: usize, q: usize,
        }
    let mut a = Vec::new();
    for i in 0..n {
        input!{l: usize}
        input!{tmp_a: [usize; l]}
        a.push(tmp_a);
    }

    for i in 0..q {
        input!{s: usize, t: usize}
        let answer = a[s-1][t-1];
        println!("{}", answer);
    }
}
