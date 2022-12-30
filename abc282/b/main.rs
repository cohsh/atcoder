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
    let s = read_stdin();
    let v: Vec<&str> = s.split_whitespace().collect();
    let n = str_to_int(v[0]);
    let m = str_to_int(v[1]);

    let mut s_num: Vec<Vec<i32>> = Vec::new();

    for _i in 0..n {
        let s_str = read_stdin();
        let mut tmp_v: Vec<i32> = Vec::new();
        for j in 0..s_str.len() {
            if s_str[j..=j] == "o".to_string() {
                tmp_v.push(1);
            } else {
                tmp_v.push(0);
            }
        }
        s_num.push(tmp_v);
    }

    let mut count = 0;

    for i in 0..n {
        for j in i+1..n {
            let mut is_match = true;
            let i_v = &s_num[i as usize];
            let j_v = &s_num[j as usize];
            for k in 0..m {
                let tmp = i_v[k as usize] + j_v[k as usize];
                if tmp == 0 {
                    is_match = false;
                }
            }
            if is_match {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
