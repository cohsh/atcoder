use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: i16};

    for i in 2..=n {
        let mut is_prime = true;
        for j in 2..i {
            if i % j == 0 { is_prime = false }
        }
        if is_prime { print!("{} ", i) }
    }
}
