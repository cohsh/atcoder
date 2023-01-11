use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{
        n: usize,
        }
    let mut answer = 1;
    for i in 1..=n {
        answer *= i;
    }
    
    println!("{}", answer);
}
