use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{h: usize, w: usize}

    let answer: usize;

    if h == 1 || w == 1 {
        answer = h * w;
    }
    else {
        answer = ((h + 1) / 2) * ((w + 1) / 2);
    }

    println!("{}", answer);
}
