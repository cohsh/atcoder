use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{
        h: usize, w: usize,
        c: [Chars; h],
    }

    let mut count = 0;
    for i in 0..w {
        count = 0;
        for j in 0..h {
            if c[j][i] == '#' {
                count += 1;
            }
        }
        println!("{}", count);
    }
}
