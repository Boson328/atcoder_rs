use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut sum = 0;
    let mut right_sum = a[n - 1];
    for i in (0..n - 1).rev() {
        sum += a[i] * right_sum;
        right_sum += a[i];
    }

    println!("{sum}");
}
