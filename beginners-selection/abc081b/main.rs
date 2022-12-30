use std::io::*;

fn read_stdin() -> String {
    let mut guess = String::new();
    stdin().read_line(&mut guess).ok();
    return guess;
}

fn str_to_int(s: String) -> i32 {
    let num: i32 = s.trim().parse().unwrap();
    return num;
}

fn split_str(s: &String) -> Vec<&str> { 
    let v : Vec<&str> = s.split_whitespace().collect();
    return v;
}

fn main() {
    // 1st input
    let n_str: String = read_stdin();
    let n = str_to_int(n_str);

    // 2nd input
    let s = read_stdin();
    let s_vec = split_str(&s);

    // -> A_i
    let mut a_vec: Vec<i32> = Vec::with_capacity(n as usize);
    let mut can_divide = true;
    let mut count = 0;
    for i in 0..n {
        a_vec.push(str_to_int(s_vec[i as usize].to_string()));
    }
    
    while can_divide {
        for i in 0..n {
            a_vec.push(str_to_int(s_vec[i as usize].to_string()));
            if a_vec[i as usize] % 2 != 0 {
                can_divide = false;
            }
        }
        if can_divide {
            for i in 0..n {
                a_vec[i as usize] = a_vec[i as usize] / 2
            }
            count += 1
        }
    }
    println!("{}", count);
}
