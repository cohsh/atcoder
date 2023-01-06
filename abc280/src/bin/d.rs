use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{k: usize}

    let mut prime_factors = prime_factorization(k);

    prime_factors.sort();
    prime_factors.reverse();
    let m = prime_factors[0];

    println!("{}", m);
}

fn prime_factorization(k: usize) -> Vec<usize> {
    let mut l = k;
    let mut answer = Vec::new();
    let mut i = 2;
    while i * i <= l {
        if l % i == 0 {
            answer.push(i);
            l /= i;
            i = 2;
        }
        i += 1;
    }
    answer.push(l);
    return answer;
}
