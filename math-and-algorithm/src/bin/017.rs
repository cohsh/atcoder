use proconio::{input, fastout};

fn gcd(a: i64, b: i64) -> i64 {
    let mut i: i64 = a;
    let mut j: i64 = b;
    while i >= 1 && j >= 1 {
        if i < j { j = j % i }
        else { i = i % j }
    }
    if i >= 1 { return i };
    return j;
}

fn lcm(a: i64, b: i64) -> i64 {
    let c = gcd(a, b);
    let result: i64;
    if a < b { result = a * (b / c) }
    else { result = b * (a / c) }
    return result;
}

#[fastout]
fn main() {
    input!{ n: i64, a: [i64; n] }
    let mut answer: i64 = 1;
    for i in 0..n {
        answer = lcm(answer, a[i as usize]);
    }
    println!("{}", answer);
}
