use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{
        mut x: f64, k: usize,
        }
    
    let mut base = 1.0;
    for i in 0..k {
        base *= 10.0;
        x /= base;
        x = x.round() * base;
    }
    
    println!("{}", x as usize);
}
