use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
}
