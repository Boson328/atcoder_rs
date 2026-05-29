use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a_b: [(usize, usize); n],
        m: usize,
        s: [String; m]
    }

    let mut s_len_map: HashMap<usize, Vec<String>> = HashMap::new();

    for si in &s {
        let s_len = si.len();
        s_len_map.entry(s_len).or_insert(vec![]).push(si.clone());
    }

    let mut full: Vec<String> = vec![];

    for i in 0..n {
        let (a, b) = a_b[i];

        let mut new_full = full.clone();
        if let Some(some_list) = s_len_map.get(&a) {
            for some in some_list {
                let s_c = &some.chars().nth(b - 1).unwrap().to_string();

                for f in full.clone() {
                    new_full.push(f + s_c);
                }
            }
        }

        full = new_full;
    }

    for si in s {
        if si.len() == n {
            full.iter().any(|&x| x == si);
        }
    }
}
