use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{s: Chars}

    let n = s.len();
    let mut v = vec![vec![]; n];
    let mut used = [false; 256];
    let mut now = 0;

    for i in 0..n {
        if s[i] == '(' {
            now += 1;
        } else if s[i] == ')' {
            for j in 0..v[now].len() {
                let m = v[now][j] as usize;
                used[m] = false;
            }
            v[now].drain(..);
            now -= 1;
        } else {
            let l = s[i] as usize;
            if used[l] {
                println!("No");
                return ();
            }
            v[now].push(s[i]);
            used[l] = true;
        }
    }
    println!("Yes");
}
