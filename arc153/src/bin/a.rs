use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize}
    let answer = nth_beautiful_number(n);

    println!("{}", answer);
}

fn nth_beautiful_number(n: usize) -> usize {
    let mut count = 0;
    for s1 in 1..10 {
        for s3 in 0..10 {
            for s4 in 0..10 {
                for s5 in 0..10 {
                    for s7 in 0..10 {
                        for s8 in 0..10 {
                            count += 1;
                            if count == n {
                                let beautiful_number = format!("{}{}{}{}{}{}{}{}{}", s1, s1, s3, s4, s5, s5, s7, s8, s7);
                                return beautiful_number.parse::<usize>().unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
    return 0;
}
