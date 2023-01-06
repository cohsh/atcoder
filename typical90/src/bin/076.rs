use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize, a: [usize; n]}

    let mut c = Cake {n: n, pieces: a, ruler: vec![0; 0], can_select: false};
    c.set_ruler();
    c.select();
    c.get_answer();
}

pub struct Cake {
    n: usize,
    pieces: Vec<usize>,
    ruler: Vec<usize>,
    can_select: bool,
}

impl Cake {
    fn set_ruler(&mut self) {
        self.can_select = true;
        self.ruler.push(0);
            for i in 1..self.n {
                self.ruler.push(self.ruler[i - 1] + self.pieces[i]);
            }
        for i in 0..self.n {
            self.ruler.push(self.ruler[self.n + i - 1] + self.pieces[i]);
        }
        if self.ruler[self.n] % 10 != 0 {
            self.can_select = false;
        }
    }

    fn select(&mut self) {
        if self.can_select {
            self.can_select = false;
            for i in 0..=self.n {
                let goal = self.ruler[i] + self.ruler[self.n] / 10;
                let mut left = i;
                let mut right = 2 * self.n + 1;
                while left <= right {
                    let mid = (left + right) / 2;
                    if self.ruler[mid] == goal {
                        self.can_select = true;
                        break;
                    } else if self.ruler[mid] > goal {
                        right = mid - 1;
                    } else if self.ruler[mid] < goal {
                        left = mid + 1;
                    }
                }
            }
        }
    }

    fn get_answer(&mut self) {
        if self.can_select {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
