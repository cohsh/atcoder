use std::io::*;

fn read_stdin() -> String {
    let mut guess = String::new();
    stdin().read_line(&mut guess).ok();
    return guess;
}

fn split_str<'a>(s: &'a String) -> Vec<&'a str> {
    let v: Vec<&str> = s.split_whitespace().collect();
    return v;
}

fn str_to_int(s: &str) -> i32 {
    let num: i32 = s.trim().parse().unwrap();
    return num;
}

fn main() {
    let n_str = read_stdin();
    let n = str_to_int(&n_str) as usize;

    let s = read_stdin();
    let v = split_str(&s);
    
    let mut a: Vec<i32> = Vec::with_capacity(n);
    let mut num: i32;
    for num_str in v {
        num = str_to_int(num_str);
        a.push(num);
    }
    a.sort();
    a.reverse();

    let mut score_alice: i32 = 0;
    let mut score_bob: i32 = 0;
    let difference: i32;
    
    for i in 0..n {
        if i % 2 == 0 {
            score_alice += a[i];
        } else {
            score_bob += a[i];
        }
    }
    difference = score_alice - score_bob;
    println!("{}", difference);
}
