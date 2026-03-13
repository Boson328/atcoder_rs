use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

// 割と愚直に解いただけ

fn main() {
    input! {
        n: u64
    }

    let sqrt = n.isqrt();
    let mut good_number: Vec<u64> = vec![];

    for i in 1..=sqrt {
        for j in i + 1..=sqrt {
            let good = i.pow(2) + j.pow(2);
            if good <= n {
                good_number.push(good);
            }
        }
    }

    let mut count_map = HashMap::new();

    for &g in &good_number {
        *count_map.entry(g).or_insert(0u32) += 1;
    }

    let good_number: Vec<String> = good_number
        .iter()
        .filter(|x| count_map[x] == 1)
        .sorted()
        .map(|x| x.to_string())
        .collect();

    let good_count = good_number.len();

    println!("{good_count}");
    println!("{}", good_number.join(" "));
}
