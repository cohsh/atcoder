use proconio::{input, fastout};

#[fastout]
fn main() {
    // input
    input!{h: usize, w: usize, a: [[usize; w]; h]}

    let mut sum_columns = vec![0; w];
    let mut sum_rows = vec![0; h];

    for i in 0..h {
        for j in 0..w {
            sum_rows[i] += a[i][j];
            sum_columns[j] += a[i][j];
        }
    }

    let mut b: usize;

    for i in 0..h {
        let mut answer = vec![];
        for j in 0..w {
            b = sum_columns[j] + sum_rows[i] - a[i][j];
            answer.push(b);
        }
        println!("{}", itertools::join(answer, " "));
    }
}
