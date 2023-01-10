use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize, a: [usize; n]}
    let mut graph = vec![ vec![0; 0]; 2 * (n+1) ];
    for i in 0..n {
        let j = 2 * (i + 1);
        graph[a[i]].push(j);
        graph[a[i]].push(j+1);
    }
    let mut depth = vec![0; 2 * (n+1) ];

    dfs(1, &mut depth, &graph);
    
    for i in 1..=(2 * n + 1) {
        println!("{}", depth[i]);
    }

}

fn dfs(start: usize, depth: &mut Vec<usize>, graph: &Vec<Vec<usize>>) {
    // depth[start] = *count;
    for &next in &graph[start] {
        depth[next] = depth[start] + 1;
        dfs(next, depth, graph);
    }
}
