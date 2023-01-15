use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!{
        n: usize,
        tmp: [[String; 2]; n],
    }
    let mut current = Vec::new();
    let mut desired = Vec::new();

    for i in 0..n {
        current.push(tmp[i][0].clone());
        desired.push(tmp[i][1].clone());
    }

    let mut desired_map = HashMap::new();

    for i in 0..n {
        let c = current[i].clone();
        let d = desired[i].clone();
        if desired_map.contains_key(&d) {
            println!("No");
            return ();
        }
        desired_map.insert(c, d);
    }
    println!("Yes");
}
