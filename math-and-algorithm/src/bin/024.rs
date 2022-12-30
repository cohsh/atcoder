use proconio::{input, fastout};

#[fastout]
fn main() {
    let mut answer: f64 = 0.0;
    input!{n: i64, a: [[i64; 2]; n]}

    for i in 0..n {
        answer += a[i as usize][1] as f64 / a[i as usize][0] as f64;
    }

    println!("{}", answer);
}
