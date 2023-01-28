use proconio::{input, fastout};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!{
        n: usize, m: usize,
        mut tmp: [[usize; 2]; m],
    }

    let mut graph = vec![vec![0; 0]; n];

    for i in 0..m {
        graph[tmp[i][0]-1].push(tmp[i][1]-1);
        graph[tmp[i][1]-1].push(tmp[i][0]-1);
    }

    let answer = is_path_graph(n, m, &graph);

    if answer {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn is_path_graph(n: usize, m: usize, graph: &Vec<Vec<usize>>) -> bool {
    if m != n-1 {
        return false;
    }

    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    queue.push_back(0);
    visited[0] = true;

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        for &neighbor in graph[v].iter() {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    for i in 0..n {
        if !visited[i] {
            return false;
        }
    }

    return true;
}
