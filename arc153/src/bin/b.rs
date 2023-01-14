use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{
        h: usize, w: usize,
        mut g: [Chars; h],
        q: usize,
        operations: [(usize, usize); q],
    }

    for (a, b) in operations {
        rotate_grid(&mut g, h, w, a, b);
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", g[i][j]);
        }
        print!("\n");
    }
}

fn rotate_grid(g: &mut Vec<Vec<char>>, h: usize, w: usize, a: usize, b: usize) {

    let mut offsets = Vec::new();
    let mut squares = Vec::new();

    offsets.push(vec![0, 0]);
    offsets.push(vec![0, b-1]);
    offsets.push(vec![a-1, 0]);
    offsets.push(vec![a-1, b-1]);

    squares.push(vec![a, b]);
    squares.push(vec![a, w-b]);
    squares.push(vec![h-a, b]);
    squares.push(vec![h-a, w-b]);

    for k in 0..4 {
        for i in 0..(squares[k][0]/2) {
            for j in 0..(squares[k][1]/2) {
                let temp = g[offsets[k][0]+i][offsets[k][1]+j];
                g[offsets[k][0]+i][offsets[k][1]+j] = g[offsets[k][0]+squares[k][0]-1-i][offsets[k][1]+squares[k][1]-1-j];
                g[offsets[k][0]+squares[k][0]-1-i][offsets[k][1]+squares[k][1]-1-j] = temp;
            }
        }
    }
}
