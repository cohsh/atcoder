use proconio::{input, fastout};

#[fastout]
fn main() {
    let mut answer: f64 = 0.0;
    input!{n: i64, b: [i64; n], r: [i64; n]}

    for i in 0..n {
        answer += (b[i as usize] + r[i as usize]) as f64 / n as f64;
    }

    println!("{}", answer);
}
