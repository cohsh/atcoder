use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{s: Chars}
    let mut answer = -1;
    for i in 0..s.len() {
        if s[i] == 'a' {
            answer = (i + 1) as isize;
        }
    }
    println!("{}", answer);
}
