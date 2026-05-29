use proconio::input;

fn main() {
    input! {
        m: usize,
        n: usize,
    }

    if m == 1 && n == 7
        || m == 3 && n == 3
        || m == 5 && n == 5
        || m == 7 && n == 7
        || m == 9 && n == 9
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
