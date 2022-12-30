use proconio::{input, fastout};
use proconio::marker::Chars;

fn win_rate(c: char, i: usize, n: usize, p: usize) -> f64 {
    let answer: f64;
    if i <= n {
        if c == 'A' {
            answer = p as f64 / 100.0;
        } else {
            answer = 1.0 - p as f64 / 100.0;
        }
    } else {
        answer = 0.5;
    }
    return answer;
}

fn win_lose(f: f64) -> String {
    let answer: String;
    if f > 0.5 {
        answer = "+".to_string();
    } else if f == 0.5 {
        answer = "0".to_string();
    } else {
        answer = "-".to_string();
    }
    return answer;
}

#[fastout]
fn main() {
    let mut answer = String::new();
    let mut win_kuro: f64;

    input!{n: usize, k: usize, p:usize, s: Chars}

    let mut dp = vec![vec![0.0; 2*k+1]; 2*k+1];
    dp[0][0] = 1.0;
    for i in 1..=k {
        dp[i][0] = 1.0;
        for j in 1..=i {
            dp[i][0] *= 1.0 - win_rate(s[j-1], j, n, p);
        }
    }

    for i in 1..=2*k {
        for j in 1..i {
            if j >= k {continue}
            dp[i][j] = dp[i-1][j-1] * win_rate(s[j-1], j, n, p)
                        + dp[i-1][j] * (1.0 - win_rate(s[j-1], j, n, p));
        }
    }

    for i in 1..=k {
        win_kuro = 0.0;
        for j in 1..=2*k {
            win_kuro += dp[j][i];
        }
        println!("{}", win_kuro);
        answer += &win_lose(win_kuro);
    }

    println!("{}", answer);
}
