use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize, h: [isize; n]}

    let mut dp = vec![0; n];

    for i in 0..n {
        if i == 0 {dp[i] = 0}
        if i == 1 {
            dp[i] = (h[i - 1] - h[i]).abs();
        }
        if i >= 2 {
            let v1 = dp[i - 1] + (h[i - 1] - h[i]).abs();
            let v2 = dp[i - 2] + (h[i - 2] - h[i]).abs();
            dp[i] = std::cmp::min(v1, v2);
        }
    }

    println!("{}", dp[n-1]);
}
