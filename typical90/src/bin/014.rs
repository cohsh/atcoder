use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize, mut a: [usize; n], mut b: [usize; n]}

    a.sort_unstable();
    b.sort_unstable();

    let mut answer = 0;

    for i in 0..n {
        answer += (a[i] as isize - b[i] as isize).abs();
    }

    println!("{}", answer);
}
