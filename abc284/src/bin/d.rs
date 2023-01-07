use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{t: usize}

    for i in 0..t {
        input!{n: usize}
        let (p, q) = prime_factorization(n);
        println!("{} {}", p, q);
    }
}

fn prime_factorization(n: usize) -> (usize, usize) {
    let mut p = 2;
    let mut q = 2;
    while (p * p <= n) && (q * q <= n) {
        if n / (p * p)
    }
    return (p, q);
}
