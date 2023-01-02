use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize, l: usize};

    let div: usize = 10_usize.pow(9) + 7;
    let mut dp = vec![1; n+1];
    for i in 1..=n {
        if i < l {dp[i] = dp[i-1]}
        else {dp[i] = (dp[i-1] + dp[i-l]) % div};
    }

    println!("{}", dp[n]);
}
