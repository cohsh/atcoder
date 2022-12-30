use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{a: [i32; 3]};
    println!("{}", a.iter().product::<i32>());
}
