use proconio::{input, fastout};
use itertools::Itertools;

#[fastout]
fn main() {
    input!{n: usize}

    for i in 0..(1 << n) {
        let mut candidate: Vec<char> = Vec::new();
        for j in (0..n).rev() {
            if (i & (1 << j)) == 0 {
                candidate.push('(');
            } else {
                candidate.push(')');
            }
        }
        let is_correct = judge(&candidate);
        if is_correct {println!("{}", candidate.iter().join(""))}
    }
}

fn judge(s: &Vec<char>) -> bool {
    let mut depth = 0;
    for i in 0..s.len() {
        if s[i] == '(' {depth += 1}
        if s[i] == ')' {depth -= 1}
        if depth < 0 {return false}
    }
    if depth == 0 {return true}
    return false;
}
