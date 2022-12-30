use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize, k: isize, a: [isize; n], b: [isize; n]}
    let mut c = 0;

    for i in 0..n { c += (a[i] - b[i]).abs() }
    c = k - c;

    if c >= 0 && c % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
