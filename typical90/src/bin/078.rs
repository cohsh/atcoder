use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize, m: usize, c: [[usize; 2]; m]}

    let mut g = vec![Vec::new(); n];
    for i in 0..m {
        g[c[i][0]-1].push(c[i][1]-1);
        g[c[i][1]-1].push(c[i][0]-1);
    }
    
    let mut count: usize;
    let mut answer = 0;
    for i in 0..n {
        count = 0;
        for j in &g[i] { if j < &i {count += 1} }
        if count == 1 {answer += 1}
    }

    println!("{}", answer);
}
