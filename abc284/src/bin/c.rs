use proconio::{input, fastout};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!{n: usize, m: usize, a: [[usize; 2]; m]}

    let mut graph = vec![vec![0; 0]; n + 1];

    for i in 0..m {
        graph[a[i][0]].push(a[i][1]);
        graph[a[i][1]].push(a[i][0]);
    }

    let mut visited = vec![false; n + 1];
    let mut count = 0;
    for i in 1..=n {
        if ! visited[i] {
            count += 1;
            let mut queue = VecDeque::new();
            queue.push_back(i);
            visited[i] = true;

            while ! queue.is_empty() {
                let node = queue.pop_front().unwrap();
                for &neighbor in &graph[node] {
                    if ! visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }
    println!("{}", count);
}
