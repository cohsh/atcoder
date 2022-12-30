use proconio::{input, fastout};

fn gcd2(a: usize, b: usize) -> usize{
    if b == 0 {return a}
    return gcd2(b, a % b);
}

fn gcd3(a: usize, b: usize, c: usize) -> usize{
    return gcd2(gcd2(a, b), c);
}

#[fastout]
fn main() {
    input!{l: [usize; 3]}

    let gcd = gcd3(l[0], l[1], l[2]);
    let mut answer = 0;

    for i in 0..3 {
        answer += l[i] / gcd - 1;
    }

    println!("{}", answer);
}
