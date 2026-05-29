use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        s: String,
    }

    //                                    i
    let mut dictionary: HashMap<char, Vec<usize>> = HashMap::new();

    let mut count: i64 = 0;

    for (idx, c) in s.chars().enumerate() {
        if let Some(i_list) = dictionary.get(&c) {
            // l-idx <= i <= r-idxで考える
            let lo = idx.saturating_sub(r);
            let hi = idx.saturating_sub(l);

            let left = i_list.partition_point(|&x| x < lo);
            let right = i_list.partition_point(|&x| x <= hi);

            count += (right - left) as i64;
            // l+i <= idx <= r+iで考えたのが間違い
            // for i in i_list {
            //     if l + i <= idx && idx <= r + i {
            //         count += 1;
            //     }
            // }
        }

        dictionary.entry(c).or_default().push(idx);
    }

    println!("{count}");
}

