use proconio::{input, fastout};

#[fastout]
fn main() {
    let div = 10_usize.pow(9) + 7;
    input!{
        n: usize, a: [[usize; 6]; n],
    }

    let mut answer = 1;
    let mut tmp: usize;
    for i in 0..n {
        tmp = 0;
        for j in 0..6 {
            tmp += a[i][j];
        }
        answer *= tmp;
        answer %= div;
    }

    println!("{}", answer);
}
