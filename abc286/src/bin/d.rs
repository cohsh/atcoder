use proconio::{input, fastout};
use proconio::marker::Chars;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!{
        n: usize, x: usize,
        tmp: [[usize; 2]; n],
        }
    let mut a = Vec::new();
    let mut b = Vec::new();
    for i in 0..n {
        a.push(tmp[i][0]);
        b.push(tmp[i][1]);
    }
    
    let answer = can_pay_exactly(a, b, x);
    
    if answer {
        println!("Yes");
    } else {
        println!("No");
    }
}


fn can_pay_exactly(a: Vec<usize>, b: Vec<usize>, x: usize) -> bool {
    let n = a.len();
    let mut dp = vec![vec![false; x as usize + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..n+1 {
        for j in 0..x as usize + 1 {
            if j >= a[i-1] as usize {
                for k in 0..b[i-1]+1 {
                    if j >= k * a[i-1] as usize && dp[i-1][j - k * a[i-1] as usize] {
                        dp[i][j] = true;
                        break;
                    }
                }
            } else {
                dp[i][j] = dp[i-1][j];
            }
        }
    }
    dp[n][x as usize]
}
