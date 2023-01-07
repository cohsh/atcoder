use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{s: Chars, t: Chars}
    let mut i = 0;
    let mut is_subsequence = false;

    if t.len() > s.len() {
        println!("No");
        return ();
    }

    while i <= s.len() - t.len() {
        if t[0] == s[i] {
            is_subsequence = true;
            for j in 1..t.len() {
                if s[i+j] != t[j] {
                    i += 1;
                    is_subsequence = false;
                    break;
                }
            }
            if is_subsequence {
                println!("Yes");
                return ();
            }
        } else {
            i += 1;
        }
    }
    println!("No");
}
