use proconio::{input, fastout};
use proconio::marker::Chars;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }
    let mut answer = max_production(n, a);

    println!("{}", answer);
}

fn max_production(n: usize, a: Vec<usize>) -> usize {
    let mut max_value = 0;
    let mut last_zero = n;
    let mut sum = 0;
    for i in 0..n {
        if a[i] == 0 {
            if i - last_zero > 1 {
                sum += a[last_zero + 1..i].iter().min().unwrap();
            }
            last_zero = i;
        }
    }
    if last_zero != n {
        sum += a[last_zero + 1..n].iter().min().unwrap();
    }
    max_value = sum;
    for i in 0..n {
        if a[i] == 0 {
            let mut sum = 0;
            let mut last_zero = i;
            for j in (i + 1)..(i + n) {
                let k = j % n;
                if a[k] == 0 {
                    if k - last_zero > 1 {
                        sum += a[last_zero + 1..k].iter().min().unwrap();
                    }
                    last_zero = k;
                }
            }
            if last_zero != i + n - 1 {
                sum += a[last_zero + 1..i + n].iter().min().unwrap();
            }
            max_value = max_value.max(sum);
        }
    }
    max_value
}
