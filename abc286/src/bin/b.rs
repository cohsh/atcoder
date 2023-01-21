use proconio::{input, fastout};
use proconio::marker::Chars;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!{
        n: usize,
        s: Chars,
    }
    let mut result = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if i + 1 < s.len() && s[i] == 'n' && s[i+1] == 'a' {
            result.push('n');
            result.push('y');
            result.push('a');
            i += 2;
        } else {
            result.push(s[i]);
            i += 1;
        }
    }

    for i in 0..result.len() {
        print!("{}", result[i]);
    }
}
