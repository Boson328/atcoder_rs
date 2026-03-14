use proconio::input;

// 気持ちよかったので解説
// x, yにs, tのA以外の文字をPushすることで、A以外の並びが同じであることを確認
// また、s, tでいくつAが並ぶかをカウントし、その差を回数とした

fn main() {
    input! {
        s: String,
        t: String,
    }

    if s == t {
        println!("0");
        return;
    }

    let mut a_s: Vec<i32> = vec![0];
    let mut a_t: Vec<i32> = vec![0];
    let mut x: String = String::new();
    let mut y: String = String::new();

    let mut i = 0;

    for si in s.chars() {
        if si == 'A' {
            a_s[i] += 1;
            continue;
        }

        x.push(si);
        i += 1;
        a_s.push(0);
    }

    let mut i = 0;

    for ti in t.chars() {
        if ti == 'A' {
            a_t[i] += 1;
            continue;
        }

        y.push(ti);
        i += 1;
        a_t.push(0);
    }

    if x != y {
        println!("-1");
        return;
    }

    let mut sum = 0;
    for i in 0..a_s.len() {
        sum += (a_s[i] - a_t[i]).abs()
    }

    println!("{sum}");
}
