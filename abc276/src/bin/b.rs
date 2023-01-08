use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: usize, m: usize,
        c: [[usize; 2]; m],
    }

    let mut graph = vec![vec![0; 0]; n+1];

    for i in 0..m {
        graph[c[i][0]].push(c[i][1]);
        graph[c[i][1]].push(c[i][0]);
    }

    for i in 1..=n {
        graph[i].sort();
        print!("{} ", graph[i].len());
        for j in 0..graph[i].len() {
            print!("{} ", graph[i][j]);
        }
        print!("\n");
    }
}
