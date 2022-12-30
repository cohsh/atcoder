use std::io::*;

fn read_stdin () -> String {
    let mut guess = String::new();
    stdin().read_line(&mut guess).ok();
    return guess;
}

fn main () {
    let s_str = read_stdin();
    let s_vec: Vec<char> = s_str.chars().collect();
    let mut sum: i8 = 0;
    for i in 0..=2 {
        let num = s_vec[i] as i8 - 48;
        sum += num;
    }
    println!("{}", sum);
}
