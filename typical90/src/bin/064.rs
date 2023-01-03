use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: usize, q: usize,
        a: [isize; n],
        b: [[isize; 3]; q],
    }
    let mut l = Vec::new();
    let mut r = Vec::new();
    let mut v = Vec::new();
    for i in 0..q {
        l.push((b[i][0] - 1) as usize);
        r.push((b[i][1] - 1) as usize);
        v.push(b[i][2]);
    }

    let mut d = vec![0isize; n];
    let mut answer = 0;
    for i in 0..n-1 {
        d[i] = a[i + 1] - a[i];
        answer += d[i].abs();
    }

    let mut before: isize;
    let mut after: isize;

    for i in 0..q {
        if l[i] >= 1 {
            before = d[l[i]-1].abs() + d[r[i]].abs();
            d[l[i]-1] += v[i];
            if r[i] < n - 1 {
                d[r[i]] -= v[i];
            }
            after = d[l[i]-1].abs() + d[r[i]].abs();
        } else {
            before = d[r[i]].abs();
            if r[i] < n - 1 {
                d[r[i]] -= v[i];
            }
            after = d[r[i]].abs();
        }
        answer += after - before;
        println!("{}", answer);
    }
}
