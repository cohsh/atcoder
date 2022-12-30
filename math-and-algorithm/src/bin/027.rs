use proconio::{input, fastout};

fn merge_sort(array: Vec<usize>) -> Vec<usize> {
    if array.len() == 1 {return array;}

    let m = array.len() / 2;
    let array1 = merge_sort((&array[0..m]).to_vec());
    let array2 = merge_sort((&array[m..array.len()]).to_vec());
    
    let mut c1 = 0;
    let mut c2 = 0;

    let mut c = Vec::new();

    while c1 != array1.len() || c2 != array2.len() {
        if c1 == array1.len() {
            c.push(array2[c2]);
            c2 += 1;
        } else if c2 == array2.len() {
            c.push(array1[c1]);
            c1 += 1;
        } else {
            if array1[c1] > array2[c2] {
                c.push(array2[c2]);
                c2 += 1;
            } else {
                c.push(array1[c1]);
                c1 += 1;
            }
        }
    }
    return c;
}

#[fastout]
fn main() {
    input!{n: usize, a: [usize; n]}

    for i in merge_sort(a) {
        print!("{} ", i)
    }
}
