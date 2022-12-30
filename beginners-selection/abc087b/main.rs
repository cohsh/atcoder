use std::io::*;

fn read_stdin() -> i32 {
    let mut guess = String::new();
    stdin().read_line(&mut guess).ok();
    let num: i32 = guess.trim().parse().unwrap();
    return num;
}

fn main() {
    let a = read_stdin();
    let b = read_stdin();
    let c = read_stdin();
    let x = read_stdin();
    let mut cases = 0;
    let mut remaining: i32;
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                remaining = x - i * 500 - j * 100 - k * 50;
                if remaining == 0 {
                    cases += 1;
                }
            }
        }
    }
    println!("{}", cases);
}
