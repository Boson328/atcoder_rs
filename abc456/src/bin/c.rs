use proconio::input;

fn main() {
    input! {
        abc:String
    }

    let mut sum_count: i64 = 0;
    let mut combo: i64 = 0;
    let mut before_char: char = '0';

    for c in abc.chars() {
        if c == before_char {
            let sum: i64 = combo * (combo + 1) / 2;
            sum_count += sum;
            combo = 0;
        }

        combo += 1;
        before_char = c;
    }

    let sum: i64 = combo * (combo + 1) / 2;
    sum_count += sum;
    sum_count %= 998244353;

    print!("{sum_count}");
}
