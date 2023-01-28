use proconio::{input, fastout};
use proconio::marker::Chars;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!{
        s: Chars,
        t: Chars,
        }
    
    let s_len = s.len();
    let t_len = t.len();
    for i in 0..=t_len {
        let mut s_prime = vec![' '; 0];
        for j in 0..i {
            s_prime.push(s[j]);
        }
        for j in s_len-t_len+i..s_len {
            s_prime.push(s[j]);
        }
        let mut match_flag = true;

        for j in 0..t_len {
            let c_s = s_prime[j];
            let t_s = t[j];
            if c_s != t_s {
                if c_s != '?' && t_s != '?' {
                    match_flag = false;
                    break;
                }
            }
        }

        if match_flag {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
