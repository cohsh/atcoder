use proconio::{input, fastout};

#[fastout]
fn main() {
    let mut answer = 0.0;
    input!{n: u32}

    for i in 1..=n {
        answer += (n as f64) / (i as f64);
    }

    println!("{}", answer);
}
