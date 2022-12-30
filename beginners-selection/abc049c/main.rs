use std::io::*;

fn read_std() -> String {
    let mut guess = String::new();
    stdin().read_line(&mut guess).ok();
    let result: String = guess.lines().collect::<String>();
    return result;
}

fn main() {
    let mut s = read_std();
    if s.len() == 0 {
        println!("NO");
        std::process::exit(0);
    }

    let patterns = ["dreamer", "eraser", "dream", "erase"];

    let mut is_match: bool;
    let mut match_pattern = String::new();

    while s.len() > "dreamer".len() {
        is_match = false;
        for pattern in patterns.iter() {
            let l_end: usize = s.len();
            let l_start = l_end - pattern.len();
            if s[l_start..l_end] == pattern.to_string() {
                is_match = true;
                match_pattern = pattern.to_string();
            }
        }
        if is_match {
            for _i in 0..match_pattern.len() {
                s.pop();
            }
        } else {
            println!("NO");
            std::process::exit(0);
        }
    }

    for pattern in patterns.iter() {
        if s == pattern.to_string() {
            for _i in 0..pattern.len() {
                s.pop();
            }
        }
    }

    if s.len() == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
