use std::io::*;

fn main() {
    let mut out_sum = 0;
    // 1st input
    let mut a_str: String = String::new();
    stdin().read_line(&mut a_str).ok();
    let a: i32 = a_str.trim().parse().unwrap();
    out_sum += a;
    
    // 2nd input
    let mut bc_str: String = String::new();
    stdin().read_line(&mut bc_str).ok();
    for num_str in bc_str.split(' ') {
        let num: i32 = num_str.trim().parse().unwrap();
        out_sum += num;
    }

    // 3rd input
    let mut s_str: String = String::new();
    stdin().read_line(&mut s_str).ok();

    print!("{} ", out_sum);
    print!("{}\n", s_str);
}
