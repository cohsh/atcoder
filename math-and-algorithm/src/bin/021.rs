use proconio::{input, fastout};

#[fastout]
fn main() {
    let answer: i64;
    input!{n: i64, r: i64}

    let mut denom: i64 = 1;
    let mut num: i64 = 1;
    for i in 0..r {
        denom *= i + 1;
        num *= n - i;
    }
    answer = num / denom;

    println!("{}", answer);
}
