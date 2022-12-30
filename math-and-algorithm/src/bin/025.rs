use proconio::{input, fastout};

#[fastout]
fn main() {
    let mut answer: f64 = 0.0;
    input!{n: i64, a: [i64; n], b: [i64; n]}

    for i in 0..n {
        answer += a[i as usize] as f64 * 1.0 / 3.0;
        answer += b[i as usize] as f64 * 2.0 / 3.0;
    }

    println!("{}", answer);
}
