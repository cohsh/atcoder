use std::io::*;

fn str_to_int(s: &str) -> i64 {
    let num: i64 = s.trim().parse().unwrap();
    return num;
}

fn read_stdin() -> String {
    let mut guess = String::new();
    stdin().read_line(&mut guess).ok();
    return guess;
}

fn main () {
    let s = read_stdin();
    let v: Vec<&str> = s.split_whitespace().collect();
    let n = str_to_int(v[0]);
    let y = str_to_int(v[1]);
    let mut m: i64;
    let mut money: i64;
    for mx in 0..=n {
        m = n - mx;
        for my in 0..=m {
            m = n - mx - my;
            money = 10000 * mx + 5000 * my + 1000 * m;
            if money == y {
                println!("{} {} {}", mx, my, m);
                std::process::exit(0);
            }
        }
    }
    println!("-1 -1 -1")
}
