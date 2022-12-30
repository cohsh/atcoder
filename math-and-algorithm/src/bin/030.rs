use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: usize, w: usize,
        a: [[usize; 2]; n]
    }

    let mut dp = vec![vec![0; 100009]; n+1];

    for i in 0..=w {
        dp[0][i] = 0;
    }

    for i in 1..=n {
        for j in 0..=w {
            if j < a[i-1][0] {
                dp[i][j] = dp[i-1][j];
            }
            if j >= a[i-1][0] {
                let k = j - a[i-1][0];
                dp[i][j] = std::cmp::max(dp[i-1][j], dp[i-1][k]+a[i-1][1]);
            }
        }
    }

    let mut answer = 0;
    for i in 0..=w {
        answer = std::cmp::max(answer, dp[n][i]);
    }
    println!("{}", answer);
}
