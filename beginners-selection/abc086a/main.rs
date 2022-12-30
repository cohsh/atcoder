use std::io::*;

fn read_stdin () -> String {
    let mut guess = String::new();
    stdin().read_line(&mut guess).ok();
    return guess;
}

fn main() {
    let stdin_str = read_stdin();
    let nums_str = stdin_str.split_whitespace();
    let mut is_even = false;
    for num_str in nums_str {
        let num : i16 = num_str.trim().parse().unwrap();
        if num % 2 == 0 {
            is_even = true;
        }
    }
    if is_even {
        println!("Even");
    } else {
        println!("Odd");
    }
}
