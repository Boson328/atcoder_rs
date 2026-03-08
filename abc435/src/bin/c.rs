use proconio::input;

// AC
fn main() {
    input! {
        n: usize, a: [usize; n]
    }

    let mut now = 0;

    while now < n {
        let fallable_now = now + a[now];

        if fallable_now >= n {
            now = n;
            break;
        }

        // max.0は相対位置, max.1は絶対位置
        let mut max = (0, fallable_now);

        for i in 1..a[now] {
            let fallable_i = now + i + a[now + i];
            if fallable_i > max.1 {
                max.0 = i;
                max.1 = fallable_i;
            }
        }

        if max.0 == 0 {
            now = fallable_now;
            break;
        }

        now += max.0;
    }

    println!("{now}");
}

//--------------------------- TLEった---------------------------
// fn main() {
//     input! {
//         n: usize, a: [usize; n]
//     }
//
//     let mut falled = vec![false; n];
//     let mut count = 0;
//
//     falled[0] = true;
//
//     for i in 0..n {
//         if falled[i] {
//             count += 1;
//             for j in 1..a[i] {
//                 if (i + j) < n {
//                     falled[i + j] = true;
//                 }
//             }
//         }
//     }
//
//     println!("{count}");
// }
