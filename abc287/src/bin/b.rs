use proconio::{input, fastout};
use proconio::marker::Chars;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!{
        n: usize, m: usize,
        s: [Chars; n],
        t: [Chars; m],
        }

    let mut answer = 0;
    let mut flag = false;

    for i in 0..n {
        flag = false;
        for j in 0..m {
            if (s[i][5] == t[j][2])
            && (s[i][4] == t[j][1])
            && (s[i][3] == t[j][0]) {
                flag = true;
            }
        }
        if flag {
            answer += 1;
        }
    }
    
    println!("{}", answer);
}
