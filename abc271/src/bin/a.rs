use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{
        n: usize,
        }
    let answer = format!("{:02X}", n);
    
    println!("{}", answer);
}
