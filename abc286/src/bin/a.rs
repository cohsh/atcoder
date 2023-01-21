use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: usize, p: usize, q: usize, r: usize, s: usize,
        mut a: [usize; n],
    }
    let segment1: Vec<usize> = a[p-1..q].to_vec();
    let segment2: Vec<usize> = a[r-1..s].to_vec();
    a[p-1..q].copy_from_slice(&segment2);
    a[r-1..s].copy_from_slice(&segment1);
    for i in 0..n {
        print!("{} ", a[i]);
    }
}
