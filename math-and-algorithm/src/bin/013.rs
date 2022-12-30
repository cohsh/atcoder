use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: i64}
    let mut i: i64 = 1;

    while i * i <= n {
        if n % i == 0 {
            println!("{}", i);
            if i != n / i {
                println!("{}", n / i);
            }
        }
        i += 1;
    }
}
