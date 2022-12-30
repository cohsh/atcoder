use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n:u8, a:[i32; n]}
    println!("{}", a.iter().sum::<i32>());
}
