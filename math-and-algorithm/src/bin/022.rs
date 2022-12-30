use proconio::{input, fastout};

#[fastout]
fn main() {
    let mut answer: i64 = 0;
    input!{n: i64, a: [i64; n]}

    let mut counts: [i64; 99999] = [0; 99999];
    let mut j: i64;

    for i in 0..n {
        j = a[i as usize] - 1;
        counts[j as usize] += 1;
    }

    for k in 0..50000 {
        answer += counts[k as usize] * counts [99998 - k as usize];
    }

    println!("{}", answer);
}
