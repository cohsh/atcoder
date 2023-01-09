use proconio::{input, fastout};
use proconio::marker::Chars;
use itertools::Itertools;

#[fastout]
fn main() {
    input!{s: [Chars; 9]}

    let mut count = 0;

    for v in (0..8).combinations_with_replacement(2) {
        for r in (0..9).combinations_with_replacement(2) {
            let r1 = r[0];
            let c1 = r[1];
            if (r1 + v[0] < 9) && (c1 + v[1] < 9) {
            if (s[r1][c1] == '#')
                && (s[r1+v[0]][c1] == '#')
                && (s[r1][c1+v[1]] == '#')
                && (s[r1+v[0]][c1+v[1]] == '#') {count += 1}
            }
        }
    }

    println!("{}", count);
}
