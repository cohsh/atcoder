use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize, mut t: usize, a: [usize; n]}
    let s: usize = a.iter().sum();
    t = t % s;

    for i in 0..n {
        if t > a[i] {
            t -= a[i];
        } else {
            println!("{} {}", i+1, t);
            return ();
        }
    }

}
