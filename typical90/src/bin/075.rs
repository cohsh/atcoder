use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{tmp: usize}
    let n = Int {value: tmp};
    let vec = n.prime_factorization();

    let mut answer = 1;
    for i in 0..=20 {
        if (1 << i) >= vec.len() {
            answer = i;
            break;
        }
    }
    println!("{}", answer);
}

pub struct Int {
    value: usize,
}

impl Int {
    pub fn prime_factorization(&self) -> Vec<usize> {
        let mut rem = self.value;
        let mut p = Vec::new();
        let mut i = 2;
        while i * i <= self.value {
            while rem % i == 0 {
                rem /= i;
                p.push(i);
            }
            i += 1;
        }
        if rem != 1 {
            p.push(rem);
        }
        return p;
    }
}
