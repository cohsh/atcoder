use proconio::{input, fastout};

fn gcd(a0: i64, b0: i64) -> i64 {
    let mut a = a0;
    let mut b = b0;
    while a >= 1 && b >= 1 {
        if a < b { b = b % a }
        else {a = a % b}
    }
    if a >= 1 {return a};
    return b;
}

#[fastout]
fn main() {
    input!{n: i64, a: [i64; n]}
    let mut answer = a[0];
    for i in 1..n {
        answer = gcd(answer, a[i as usize]);
    }
    println!("{}", answer);
}
