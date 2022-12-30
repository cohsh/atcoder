use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {n: Chars, k: usize}
    
    let mut answer: Vec<char> = n;
    for _i in 0..k {
        let j: usize = encode(answer);
        answer = decode(j);
    }

    println!("{}", answer.iter().collect::<String>());
}

fn encode(n: Vec<char>) -> usize {
    let mut answer = 0;
    for i in (0..n.len()).rev() {
        answer += (n[i] as usize - '0' as usize) * 8_usize.pow((n.len() - i - 1) as u32)
    }
    return answer;
}

fn decode(n: usize) -> Vec<char> {
    let mut answer: Vec<char> = Vec::new();
    let mut m = n;
    let mut i: usize;
    
    if m == 0 {
        return vec!['0'];
    }

    while m > 0 {
        i = m % 9;
        if i == 8 {i = 5};
        let c = std::char::from_digit(i as u32, 10).unwrap();
        answer.push(c);
        m /= 9;
    }

    let mut reverse: Vec<char> = Vec::new();

    for j in (0..answer.len()).rev() {
        reverse.push(answer[j]);
    }

    return reverse;
}
