use proconio::{input, fastout};

const M: usize = 46;

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    let mut rem = vec![vec![0; M]; 3];
    for i in 0..n {
            rem[0][a[i] % M] += 1;
            rem[1][b[i] % M] += 1;
            rem[2][c[i] % M] += 1;
    }
    let mut answer: u128 = 0;
    for i in 0..M {
        for j in 0..M {
            for k in 0..M {
                if (i + j + k) % M == 0 {
                    answer += rem[0][i] * rem[1][j] * rem[2][k];
                }
            }
        }
    }
    println!("{}", answer);
}
