use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize, a: [usize; n]}
    
    let mut dp = vec![0; n];

    dp[0] = a[0];
    dp[1] = a[1];

    for i in 2..n {
        dp[i] = dp[i-1].max(dp[i-2]+a[i]);
    }

    println!("{}", dp[n-1]);
}
