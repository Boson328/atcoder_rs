use proconio::input;

// 姑息な真似しなくても普通に解けた

fn min(a: usize, b: usize) -> usize {
    if a > b {
        return b;
    }

    a
}

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        cxv: [(char, usize, usize); q],
    }

    let mut min_sum = 0;
    let mut min_vec: Vec<usize> = vec![0; n];

    for i in 0..n {
        let mi = min(a[i], b[i]);
        min_vec[i] = mi;
        min_sum += mi;
    }

    for (c, x, v) in cxv {
        let x = x - 1;

        let mi = if c == 'A' {
            a[x] = v;
            min(b[x], v)
        } else {
            b[x] = v;
            min(a[x], v)
        };

        if min_vec[x] != mi {
            min_sum -= min_vec[x];
            min_sum += mi;

            min_vec[x] = mi;
        }

        println!("{min_sum}");
    }
}
