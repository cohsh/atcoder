use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{
        h: usize, w: usize,
        s: [Chars; h],
        t: [Chars; h],
    }
    let mut s_trans = vec![vec![' '; 0]; w];
    let mut t_trans = vec![vec![' '; 0]; w];

    for i in 0..w {
        for j in 0..h {
            s_trans[i].push(s[j][i]);
            t_trans[i].push(t[j][i]);
        }
    }

    s_trans.sort();
    t_trans.sort();

    if s_trans == t_trans {
        println!("Yes");
    } else {
        println!("No");
    }
}
