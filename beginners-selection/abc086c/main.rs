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
    let n_str = read_stdin();
    let n = str_to_int(&n_str);
    let mut t0: i32 = 0;
    let mut dt: i32;
    let mut can_do_travel = true;
    let mut x0 = 0;
    let mut y0 = 0;
    for _i in 0..n {
        let s = read_stdin();
        let v: Vec<&str> = s.split_whitespace().collect();
        let t: i32 = str_to_int(v[0]);
        let x: i32 = str_to_int(v[1]);
        let y: i32 = str_to_int(v[2]);

        dt = t - t0;
        let d = (x - x0).abs() + (y - y0).abs();
        if d > dt {
            can_do_travel = false;
        }

        if (dt - d) % 2 != 0 {
            can_do_travel = false;
        }
        t0 = t;
        x0 = x;
        y0 = y;
    }
    if can_do_travel {
        println!("Yes");
    } else {
        println!("No");
    }
}
