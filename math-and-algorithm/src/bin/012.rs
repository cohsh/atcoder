use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: i64}
    let mut is_prime = true;
    
    let mut i: i64 = 2;

    while i * i <= n {
        if n % i == 0 { is_prime = false }
        i += 1;
    }

    if is_prime {
        println!("Yes");
    } else {
        println!("No");
    }
}
