use proconio::{input, fastout};
use itertools::Itertools;

#[fastout]
fn main() {
    input!{n: usize, p: [usize; n]}
    let perms: Vec<Vec<usize>> = (1..=n).permutations(n).collect();
    let result = perms.binary_search(&p);
    if result.is_ok() {
        let i: usize = result.unwrap();
        let answer = perms[i-1].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" ");
        println!("{}", answer);
    }
}
