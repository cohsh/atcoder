use proconio::{input, fastout};
use std::f64::consts::PI;

#[fastout]
fn main() {
    input!{
        t: f64,
        l: f64, x: f64, y: f64,
        q: usize,
        e: [f64; q],
    }

    let mut r = vec![0.0, 0.0, 0.0];
    for i in 0..q {
        r = position(l, t, e[i]);
        println!("{}", angle(&r, x, y));
    }
}

fn position(l: f64, t: f64, e: f64) -> Vec<f64> {
    let mut answer = vec![0.0, 0.0, 0.0];
    answer[1] = - (l / 2.0) * (e / t * 2.0 * PI).sin();
    answer[2] = (l / 2.0) -(l / 2.0) * (e / t * 2.0 * PI).cos();
    return answer;
}

fn angle(r: &Vec<f64>, x: f64, y: f64) -> f64 {
    let d1 = ((r[0] - x).powf(2.0) + (r[1] - y).powf(2.0)).sqrt();
    let d2 = r[2];
    let theta = d2.atan2(d1);
    return theta * 180.0 / PI;
}
