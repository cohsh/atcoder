use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: usize,
        mut a: [usize; n],
    }
    let b = vec![1; n];
    let mut count = 0;

    while a != b {
    }

    println!("{}", count);
}

fn prime_factorization(n: usize) -> Vec<(usize, usize)> {
    let mut answer = Vec::new();
    let mut m = n;
    let mut i = 2;
    let mut count = 0;
    while i * i <= n {
        count = 0;
        while m % i == 0 {
            m /= i;
            count += 1;
        }
        if count != 0 {
            answer.push((i, count));
        }
        i += 1;
    }
}
