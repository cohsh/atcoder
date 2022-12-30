use proconio::{input, fastout};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        q: usize,
        a: [[usize; 2]; q],
    }

    let mut c: VecDeque<usize> = VecDeque::new();

    for i in 0..q {
        match a[i][0] {
            1 => c.push_front(a[i][1]),
            2 => c.push_back(a[i][1]),
            3 => println!("{}", c[a[i][1]-1]),
            _ => (),
        }
    }
}
