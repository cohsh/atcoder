use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{a: u128, b: u128}

    let answer = a * (b / gcd(a, b));
    if answer <= 10_u128.pow(18) {
        println!("{}", answer)
    } else {
        println!("Large")
    }
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {return a}
    return gcd(b, a % b);
}
