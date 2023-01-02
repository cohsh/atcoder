use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: usize, q: usize,
        mut a: [usize; n],
        b: [[isize; 3]; q],
    }
    
    let mut shift = 0;
    for i in 0..q {
        let t = b[i][0];
        let x = (b[i][1] - 1 + shift) % n as isize;
        let y = (b[i][2] - 1 + shift) % n as isize;
        match t {
            1 => a.swap(x as usize, y as usize),
            2 => shift = (shift + n as isize - 1) % n as isize,
            3 => println!("{}", a[x as usize]),
            _ => (),
        }
    }
}
