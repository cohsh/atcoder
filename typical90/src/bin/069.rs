use proconio::{input, fastout};

const DIV: usize = 1000000007;

#[fastout]
fn main() {
    input!{n: usize, k: usize}

    let answer: usize;
    if k == 1 {
        if n == 1 {
            answer = 1;
        } else {
            answer = 0;
        }
    } else if n == 1 {
        answer = k % DIV;
    } else if n == 2 {
        answer = k * (k - 1) % DIV;
    } else {
        answer = k * (k - 1) % DIV * pow(k - 2, n - 2) % DIV;
    }

    println!("{}", answer);
}

fn pow(mut a: usize, mut b: usize) -> usize {
    let mut answer = 1;
    while b != 0 {
        if b % 2 == 1 {
            answer = answer * a % DIV;
        }
        a = a * a % DIV;
        b /= 2;
    }
    return answer;
}
