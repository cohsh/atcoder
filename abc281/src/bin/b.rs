use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{mut s: Chars}
    if s.len() != 8 {
        println!("No");
        return ();
    }
    let a = vec![s.len() - 1, 0];
    for i in a {
        if ! s[i].is_ascii_alphabetic() || ! s[i].is_ascii_uppercase() {
            println!("No");
            return ();
        }
        s.remove(i);
    }
    if s.len() == 0 {
        println!("No");
        return ();
    }
    let mut t = String::new();
    for i in 0..s.len() {
        t += &s[i].to_string();
    }
    let n: usize = t.parse().unwrap_or(0);
    if n < 100000 || n > 999999 {
        println!("No");
        return ();
    }
    println!("Yes");
}
