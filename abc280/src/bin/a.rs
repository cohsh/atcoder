use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{
        h: usize, w: usize,
        s: [Chars; h],
    }
    
    let mut answer = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                answer += 1;
            }
        }
    }

    println!("{}", answer);
}
