use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {a:[i16;3]};
    println!("{}", a[0]+a[1]+a[2]);
}
