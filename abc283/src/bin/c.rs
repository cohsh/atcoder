use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{s: Chars}

    let mut r = '1';
    let mut count = 0;

    for i in 0..s.len() {
        if s[i] == '0' {
            if r == '0' {
                count += 0;
                r = '1';
            } else {
                count += 1;
                r = s[i];
            }
        } else {
            count += 1;
            r = s[i];
        }
    }

    println!("{}", count);
}
