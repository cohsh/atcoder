use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize, x: usize, mut a: [usize; n]}

    a.sort();

    let mut left = 1;
    let mut right = n;
    while left <= right {
        let mid = (left + right) / 2;
        if a[mid-1] == x {
            println!("Yes");
            return ();
        }
        if a[mid-1] > x {
            right = mid - 1;
        }
        if a[mid-1] < x {
            left = mid + 1;
        }
    }

    println!("No");
}
