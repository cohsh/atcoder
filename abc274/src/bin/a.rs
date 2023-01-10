use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{a: f64, b: f64}
    let mut answer = b / a;
    println!("{:.3}", answer);
}
