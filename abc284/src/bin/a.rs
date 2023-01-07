use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{n: usize, s: [String; n]}
    
    for i in (0..n).rev() {
        println!("{}", s[i]);
    }
}
