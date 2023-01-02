use proconio::{input, fastout};
use itertools::Itertools;
use std::cmp;

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        b: [[usize; 2]; m],
    }
    let mut answer: isize = 1 << 30;
    let mut dangerous = vec![vec![false; n]; n];
    
    for i in 0..m {
        dangerous[b[i][0]-1][b[i][1]-1] = true;
        dangerous[b[i][1]-1][b[i][0]-1] = true;
    }

    for perm in (0..n).permutations(n) {
        let mut can_goal = true;
        let mut sum = 0;
        for i in 0..n-1 {
            if dangerous[perm[i]][perm[i+1]] == true {can_goal = false}
        }
        for i in 0..n {
            sum += a[perm[i]][i];
        }
        if can_goal {answer = cmp::min(answer, sum as isize)}
    }
    if answer == 1 << 30 {answer = -1}
    println!("{}", answer);
}
