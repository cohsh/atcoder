use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize, mut a: [usize; n], q: usize}

    let mut j: usize;
    let mut k: usize;
    let mut x: usize;
    let mut s: usize;

    for i in 0..q {
        input!{j: usize}
        if j == 1 {
            input!{k: usize, x: usize}
            a[k-1] = x;
        } else if j == 2 {
            input!{s: usize}
            println!("{}", a[s-1]);
        }
    }
}
