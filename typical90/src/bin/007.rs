use proconio::{input, fastout};
use std::cmp;

#[fastout]
fn main() {
    input!{n: usize, mut a: [usize; n], q: usize, b: [usize; q]}

    a.sort_unstable();

    for i in 0..q {
        let answer = binary_search(&a, b[i]);
        println!("{}", answer);
    }
}

fn binary_search(list: &Vec<usize>, item: usize) -> usize {
    let mut left: isize = 1;
    let mut right: isize = list.len() as isize;
    let mut mid: isize = 0;
    while left <= right {
        mid = (left + right) / 2;
        let guess = list[(mid-1) as usize];
        if guess == item {return 0}
        if guess < item {left = mid + 1}
        else {right = mid - 1}
    }
    let canditate1 = (list[(mid-1) as usize] as isize - item as isize).abs() as usize;
    let mut canditate2 = canditate1;
    let mut canditate3 = canditate1;
    if mid > 2 {
        canditate2 = (list[(mid-2) as usize] as isize - item as isize).abs() as usize;
    }
    if mid < list.len() as isize {
        canditate3 = (list[mid as usize] as isize - item as isize).abs() as usize;
    }
    return cmp::min(canditate1, cmp::min(canditate2, canditate3));
}
