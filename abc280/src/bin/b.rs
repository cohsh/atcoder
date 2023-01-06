use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize, s: [isize; n]}

    print!("{} ", s[0]);
    for i in 1..n {
        print!("{} ", s[i]-s[i-1]);
    }
}
