use std::io::*;

fn str_to_int(s: String) -> i32 {
    let num: i32 = s.trim().parse().unwrap();
    return num;
}

fn read_stdin() -> i32 {
    let mut guess = String::new();
    stdin().read_line(&mut guess).ok();
    let num = str_to_int(guess);
    return num;
}


fn main() {
    let n = read_stdin();
    let mut d_vec: Vec<i32> = Vec::with_capacity(n as usize);
    for _i in 0..n {
        let d = read_stdin();
        d_vec.push(d);
    }
    d_vec.sort();
    d_vec.dedup();
    
    println!("{}", d_vec.len());
}
