use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{s: Chars}

    let mut answer = 0;

    for i in 0..s.len() {
        match s[i] {
            'v' => answer += 1,
            'w' => answer += 2,
             _  => (),
        }
    }
    println!("{}", answer);
}
