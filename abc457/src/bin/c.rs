use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        la: [[u64]; n],
        c: [u64; n]
    }

    let mut current_len: u64 = 0;
    let k = k - 1;

    for i in 0..n {
        let l = la[i].len() as u64;

        let len = l * c[i];

        if current_len + len > k {
            let idx = (k - current_len) % l;
            println!("{}", la[i][idx as usize]);
            break;
        }

        current_len += len;
    }
}
