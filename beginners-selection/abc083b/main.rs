use std::io::*;

fn read_stdin() -> String{
    let mut guess = String::new();
    stdin().read_line(&mut guess).ok();
    return guess;
}

fn main() {
    let s = read_stdin();
    let v : Vec<&str> = s.split_whitespace().collect();
    let n: i32 = v[0].parse().unwrap();
    let a: i32 = v[1].parse().unwrap();
    let b: i32 = v[2].parse().unwrap();
    let mut sum: i32 = 0;
    for i in 1..=n {
        let i_str: String = i.to_string();
        let i_vec: Vec<char> = i_str.chars().collect::<Vec<char>>();
        let mut sum_digit: i32 = 0;
        for d in i_vec {
            let num: i32 = d as i32 - 48;
            sum_digit += num;
        }
        if sum_digit >= a && sum_digit <= b {
            sum += i;
        }
    }
    println!("{}", sum);
}
