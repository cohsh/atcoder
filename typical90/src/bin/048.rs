use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: usize, k: usize,
        c: [[isize; 2]; n],
    }
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(c[i][1]);
        vec.push(c[i][0] - c[i][1]);
    }

    let mut answer = 0;
    vec.sort();
    vec.reverse();
    for i in 0..k {
        answer += vec[i];
    }

    println!("{}", answer);
}
