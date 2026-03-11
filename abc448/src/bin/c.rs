// C - Except and Min

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut a_sort = a.clone();
    a_sort.sort();
    let mut pri: Vec<String> = vec![];

    for _ in 0..q {
        input! {
            k: usize,
            b: [Usize1; k]
        }
        let mut ba: Vec<usize> = b.iter().map(|&b_i| a[b_i]).collect();
        ba.sort();

        for i in 0..n {
            if a_sort[i] != ba[i] {
                pri.push(format!("{}", a_sort[i]));
                break;
            }

            if i == k - 1 {
                pri.push(format!("{}", a_sort[i + 1]));
                break;
            }
        }
    }

    for p in pri {
        println!("{p}");
    }
}

