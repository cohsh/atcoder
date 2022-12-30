use std::io::*;

fn read_stdin() -> String {
    let mut guess = String::new();
    stdin().read_line(&mut guess).ok();
    return guess;
}

fn str_to_int(s: &str) -> i32 {
    let num: i32 = s.trim().parse().unwrap();
    return num;
}

fn main() {
    let n_str = read_stdin();
    let n = str_to_int(&n_str);

    let mut in_braket = false;

    let s = read_stdin();

    let mut results = String::new();

    for i in s.as_str().chars() {
        let mut j = String::new(); 
        let i_str = i.to_string();
        if i_str == "\"" {
            if in_braket {
                in_braket = false;
            } else {
                in_braket = true;
            }
        }
        if i_str == "," && !in_braket {
            j = ".".to_string();
        } else {
            j = i_str;
        }
        results += &j;
    }
    println!("{}", results);
}
