use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n],
        }

    let mut b = a.to_vec();
    b.sort();
    b.reverse();
    b.dedup();
    let mut weight = HashMap::new();
    for i in 0..b.len() {
        weight.insert(b[i], 0);
    }

    for i in 0..a.len() {
        *weight.get_mut(&a[i]).unwrap() += 1;
    }


    for i in 0..b.len() {
        println!("{}", weight[&b[i]]);
    }
    for i in 0..(n - b.len()) {
        println!("0");
    }
}
