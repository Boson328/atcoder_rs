use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]  // u128は不要、u64で十分
    }

    // 各値の出現回数をカウント
    let mut count = std::collections::HashMap::new();
    for &x in &a {
        *count.entry(x).or_insert(0u32) += 1;
    }

    // 重複していない値の中で最大のものを探す
    let max_unique = a.iter().filter(|&&x| count[&x] == 1).max();

    match max_unique {
        None => println!("-1"),
        Some(&max) => {
            let person = a.iter().position(|&x| x == max).unwrap() + 1;
            println!("{person}");
        }
    }
}

