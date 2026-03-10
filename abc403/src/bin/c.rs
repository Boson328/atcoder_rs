use std::collections::HashSet;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
    }

    let mut pri: Vec<&str> = vec![];
    let mut is_admin = vec![false; n];
    let mut view_list: Vec<HashSet<usize>> = vec![HashSet::new(); n];

    for _ in 0..q {
        input! {
            q_num: u8,
        }

        match q_num {
            1 => {
                input! {x: Usize1, y: Usize1}
                view_list[x].insert(y);
            }
            2 => {
                input! {x: Usize1}
                is_admin[x] = true;
            }
            _ => {
                input! {x: Usize1, y:Usize1}
                if is_admin[x] || view_list[x].contains(&y) {
                    pri.push("Yes");
                } else {
                    pri.push("No");
                }
            }
        }
    }

    for y_n in pri {
        println!("{y_n}");
    }
}
