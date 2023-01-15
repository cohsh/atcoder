use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{
        s: Chars,
        }

    let base: usize = 26;
    let n = s.len();
    let mut ans = 0;
    for i in 0..n {
        ans += (s[i] as usize - 'A' as usize + 1) * base.pow((n-i-1) as u32);
    }
    
    println!("{}", ans);
}
