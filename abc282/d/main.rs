// time is up
use std::io::*;

fn read_stdin() -> String {
    let mut guess = String::new();
    stdin().read_line(&mut guess).ok();
    return guess;
}

fn str_to_int(s: &str) -> i32 {
    let num = s.trim().parse().unwrap();
    return num;
}

fn main() {
    let s1 = read_stdin();
    let v1: Vec<&str> = s1.split_whitespace().collect();
    let n = str_to_int(v1[0]);
    let m = str_to_int(v1[1]);

    let mut g1: Vec<[i32; 2]> = Vec::new();
    let mut g2: Vec<[i32; 2]> = Vec::new();

    for i in 0..m {
        let s2 = read_stdin();
        let v2: Vec<&str> = s2.split_whitespace().collect();
        let u = str_to_int(v2[0]);
        let v = str_to_int(v2[1]);
        g1.push([u, v]); 
    }
    
    let mut count = 0;

    for i in 0..n {
        for j in i..n {
            if !(g1.iter().any(|e| e==&[i,j])) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
