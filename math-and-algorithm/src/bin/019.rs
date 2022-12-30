use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: i64, a: [i64; n]}
    let mut counts: [i64; 3] = [0, 0, 0];
    for i in 0..n {
        match a[i as usize] {
            1 => counts[0] += 1,
            2 => counts[1] += 1,
            3 => counts[2] += 1,
            _ => (),
        }
    }

    let mut answer: i64 = 0;
    for j in 0..3 {
        if counts[j] > 1 {
            answer += counts[j] * (counts[j] - 1) / 2;
        }
    }

    println!("{}", answer);
}
