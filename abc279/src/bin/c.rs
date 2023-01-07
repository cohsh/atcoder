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

    let mut can_do_same: bool;
    for i in 0..w {
        can_do_same = false;
        for j in 0..w {
            if t_trans[i] == s_trans[j] {
                let tmp1 = s_trans[i].to_vec();
                let tmp2 = s_trans[j].to_vec();
                s_trans[i] = tmp2;
                s_trans[j] = tmp1;
                can_do_same = true;
                break;
            }
        }
        if ! can_do_same {
            println!("No");
            return ();
        }
    }

    println!("Yes");
}
