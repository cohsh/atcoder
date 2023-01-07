use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{t: usize}

    for i in 0..t {
        input!{n: usize, a: [usize; n]}
        let mut count = 0;
        for j in 0..n {
            if a[j] % 2 != 0 {count += 1}
        }
        println!("{}", count);
    }
}
