use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n],
        }
    let mut answer = 0;
    for i in 0..n {
        answer += a[i];
    }
    
    println!("{}", answer);
}
