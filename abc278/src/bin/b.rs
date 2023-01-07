use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{mut h: usize, mut  m: usize}

    let mut exc = exchange(h, m);

    while ! judge(exc) {
       let mut minutes = h * 60 + m ;
       minutes += 1;
       minutes %= 24 * 60;
       h = minutes / 60;
       m = minutes % 60;
       exc = exchange(h, m);
    }

    println!("{} {}", h, m);
}

fn judge(exc: Vec<usize>) -> bool {
    let answer: bool;
    let h = exc[0];
    let m = exc[1];

    if h <= 23 && m <= 59 {
        answer = true;
    } else {
        answer = false;
    }
    return answer;
}

fn exchange(h: usize, m: usize) -> Vec<usize> {
    let h_ex = (h / 10) * 10 + m / 10;
    let m_ex = (h % 10) * 10 + m % 10;
    return vec![h_ex, m_ex];
}
