use proconio::{input, fastout};

#[fastout]
fn main() {
    // input
    input!{n: i32, x: i32, y: i32}

    // solve
    let mut cnt = 0;
    for i in 1..=n {
        if i % x == 0 || i % y == 0 {cnt+=1}
    }

    // output
    println!("{}", cnt);
}
