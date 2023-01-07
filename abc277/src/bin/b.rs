use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    let c1 = vec!['H', 'D', 'C', 'S'];
    let c2 = vec!['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K'];

    input!{
        n: usize,
        s: [Chars; n],
    }

    let mut flag: bool;
    for i in 0..n {
        flag = false;
            for j in 0..c1.len() {
                if s[i][0] == c1[j] {
                    flag = true;
                }
            }

        if ! flag {
            println!("No");
            return ();
        }
    }
    for i in 0..n {
        flag = false;
        for j in 0..c2.len() {
            if s[i][1] == c2[j] {
                flag = true;
            }
        }
        if ! flag {
            println!("No");
            return ();
        }
    }

    flag = true;
    for i in 0..n {
        if i < n - 1 {
            for j in i+1..n {
                if s[i] == s[j] {flag = false}
            }
        }
    }
    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
