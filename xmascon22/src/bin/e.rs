use proconio::{input, fastout};

#[fastout]
fn main() {
    let mut answer = 1.0;
    input!{n: usize, p: [usize; n]}

    for i in 0..n {
        answer *= p[i] as f64 / 100.0;
    }

    println!("{}", answer);
}
