use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize, q: usize}

    let mut relations = vec![vec![false; n]; n];

    for _i in 0..q {
        input!{t: usize, mut a: usize, mut b: usize}
        a -= 1;
        b -= 1;
        match t {
            1 => relations[a][b] = true,
            2 => relations[a][b] = false,
            3 => check(relations[a][b], relations[b][a]),
            _ => (),
        }
    }
}

fn check(f1: bool, f2: bool) {
    if f1 && f2 {
        println!("Yes");
    } else {
        println!("No");
    }
    return ();
}
