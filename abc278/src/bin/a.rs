use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: usize, k: usize,
        mut a: [usize; n],
    }

    for i in 0..k {
        a.remove(0);
        a.push(0);
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
}
