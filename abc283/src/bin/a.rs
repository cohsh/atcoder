use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{a: usize, b: u32}
    println!("{}", a.pow(b));
}
