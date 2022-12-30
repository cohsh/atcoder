use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: i64, a: [i64; n]}
    let mut counts: [i64; 4] = [0, 0, 0, 0];
    
    for i in 0..n {
        match a[i as usize] {
            100 => counts[0] += 1,
            200 => counts[1] += 1,
            300 => counts[2] += 1,
            400 => counts[3] += 1,
            _ => (),
        }
    }
    let answer = counts[0] * counts[3] + counts[1] * counts[2];

    println!("{}", answer);
}
