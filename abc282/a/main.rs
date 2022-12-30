use std::io::*;

const ALPHABET: [&str; 26] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];


fn read_stdin() -> i32 {
    let mut guess = String::new();
    stdin().read_line(&mut guess).ok();
    let num: i32 = guess.trim().parse().unwrap();
    return num;
}

fn main() {
    let k = read_stdin();
    let mut s = String::new();
    for i in 0..k {
        s += ALPHABET[i as usize];
    }
    println!("{}", s);
}
