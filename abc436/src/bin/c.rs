use std::collections::HashSet;

use proconio::{input, marker::Usize1};

// 愚直に解いたつもりだけど行けた

fn main() {
    input! {
        n: u64,
        m: usize,
        rc: [(Usize1, Usize1); m]
    }

    let mut placed: HashSet<(u64, u64)> = HashSet::new();

    let mut block_count = 0;

    for (r, c) in rc {
        let r = r as u64;
        let c = c as u64;
        if !placed.contains(&(r, c))
            && !placed.contains(&(r + 1, c))
            && !placed.contains(&(r, c + 1))
            && !placed.contains(&(r + 1, c + 1))
        {
            placed.insert((r, c));
            placed.insert((r + 1, c));
            placed.insert((r, c + 1));
            placed.insert((r + 1, c + 1));
            block_count += 1;
        }
    }

    println!("{block_count}");
}
