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
    let mut i = 2;
    let mut a = 2;
    while i * i * i <= n {
        if n % i == 0 {
            a = i;
            break;
        }
        i += 1;
    }

    let p: usize;
    let q: usize;

    if (n / a) % a == 0 {
        p = a;
        q = (n / a) / a;
    } else {
        p = ((n / a) as f64).sqrt() as usize;
        q = a;
    }
    return (p, q);
}
