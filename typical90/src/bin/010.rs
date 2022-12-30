use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize, cp: [[usize; 2]; n], q: usize, lr: [[usize; 2]; q]}

    let mut sa = vec![0; n+1];
    let mut sb = vec![0; n+1];

    for i in 0..n {
        sa[i+1] = sa[i];
        sb[i+1] = sb[i];
        if cp[i][0] == 1 {sa[i+1] += cp[i][1]}
        if cp[i][0] == 2 {sb[i+1] += cp[i][1]}
    }

    for i in 0..q {
        let l = lr[i][0];
        let r = lr[i][1];
        let a = sa[r] - sa[l-1];
        let b = sb[r] - sb[l-1];
        println!("{} {}", a, b);
    }
}
