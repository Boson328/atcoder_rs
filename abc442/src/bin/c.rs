use itertools::Itertools;
use proconio::input;

fn combination3(n: usize) -> usize {
    n * (n - 1) * (n - 2) / 6
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    if m == 0 {
        println!("{}", vec![combination3(n - 1); n].iter().join(" "));
        return;
    }

    input! {
        ab: [(usize, usize); m]
    }

    let mut unrelated = vec![n - 1; n];
    let mut unrelated_comb = vec![0; n];

    for (a, b) in ab {
        unrelated[a - 1] -= 1;
        unrelated[b - 1] -= 1;
    }

    for i in 0..n {
        if unrelated[i] >= 3 {
            unrelated_comb[i] = combination3(unrelated[i]);
        }
    }

    println!("{}", unrelated_comb.iter().join(" "));
}
