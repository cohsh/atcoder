use proconio::{input, fastout};
use std::collections::VecDeque;
use std::cmp;

#[fastout]
fn main() {
    input!{
        n: usize, m: usize,
        a: [[usize; 2]; m],
    }
    let mut graph = vec![vec![0; 0]; n+1];
    for i in 0..m {
        graph[a[i][0]].push(a[i][1]);
        graph[a[i][1]].push(a[i][0]);
    }

    let mut count = 0;
    let mut visited = vec![false; n+1];

    let mut stack = VecDeque::new();
    stack.push_back(1);
    visited[1] = true;

    let mut path = vec![0; 0];
    while let Some(u) = stack.pop_back() {
        path.push(u);

        for &v in &graph[u] {
            if !visited[v] {
                stack.push_back(v);
                visited[v] = true;
            }
        }
        if path.len() > 0 && graph[path[path.len()-1]].len() == 1 {
            count += 1;
        }
    }
    println!("{}", cmp::min(count, 10_usize.pow(6)));
}
