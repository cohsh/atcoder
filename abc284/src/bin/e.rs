use proconio::{input, fastout};
use std::process::exit;

#[fastout]
fn main() {
    input!{
        n: usize, m: usize,
        a: [[usize; 2]; m],
    }
    let mut graph = vec![vec![0; 0]; n];
    for i in 0..m {
        graph[a[i][0]-1].push(a[i][1]-1);
        graph[a[i][1]-1].push(a[i][0]-1);
    }

    let mut count = 1;
    let mut visited = vec![false; n];

    dfs(0, &mut visited, &graph, &mut count);
    println!("{}", count);

}

fn dfs(start: usize, visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>, count: &mut usize) {
    if *count >= 1000000 {
        println!("1000000");
        exit(0);
    }
    visited[start] = true;
    for &next in &graph[start] {
        if visited[next] {
            continue;
        }
        *count += 1;
        dfs(next, visited, graph, count);
        visited[next] = false;
    }
}
