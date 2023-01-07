use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{k: usize}
}

fn prime_factorization(k: usize) -> Vec<(usize, usize)> {
    let mut answer = Vec::new();
    let mut i = 2;
    let mut count: usize;
    let mut l = k;
    while i * i <= k {
        count = 0;
        while l % i == 0 {
            l /= i;
            count += 1;
        }
        if count != 0 {
            answer.push((i, count));
                i += 1;
        }
    }
    return answer;
}
