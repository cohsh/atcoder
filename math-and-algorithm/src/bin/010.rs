use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: i64}

    let answer: i64 = (1..=n).into_iter().product();

    println!("{}", answer);
}
