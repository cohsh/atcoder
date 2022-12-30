use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!{n: usize, s: [String; n]}
    let mut map: HashMap<String, usize> = HashMap::new();

    for i in 0..n {
        if ! map.contains_key(&s[i]) {
            let t = &s[i];
            map.insert(t.to_string(), 0);
            println!("{}", i+1);
        }
    }
}
