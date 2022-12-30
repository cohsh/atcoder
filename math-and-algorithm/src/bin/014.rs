use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: i64};
    let mut i: i64 = 2;
    let mut m: i64 = n;

    while i * i <= m {
        if m % i == 0 {
            m /= i;
            print!("{} ", i);
            i = 2;
        } else {
            i += 1;
        }
    }
    print!("{}", m);
}
