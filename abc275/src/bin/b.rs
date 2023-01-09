use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{mut n: [u128; 6]}
    let div: u128 = 998244353;
    
    for i in 0..n.len() {
        n[i] %= div;
    }

    let mut tmp1 = n[0];
    tmp1 *= n[1];
    tmp1 %= div;
    tmp1 *= n[2];
    tmp1 %= div;

    let mut tmp2 = n[3];
    tmp2 *= n[4];
    tmp2 %= div;
    tmp2 *= n[5];
    tmp2 %= div;

    let mut answer = (tmp1 - tmp2 + div) % div;

    println!("{}", answer);
}
