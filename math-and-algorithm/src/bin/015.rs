use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{mut a: i64, mut b: i64}
    let answer: i64;

    while a >= 1 && b >= 1 {
        if a < b { b = b % a }
        else { a = a % b }
    }
    if a >= 1 { answer = a }
    else { answer = b }

    println!("{}", answer);
}
