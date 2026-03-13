// C - Distance Indicators

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }

    let mut dict_count: HashMap<i32, u64> = HashMap::new();

    let mut count = 0;

    for i in 0..n {
        let i_ai = i as i32 + a[i];
        let j_aj = i as i32 - a[i];
        if let Some(c) = dict_count.get(&j_aj) {
            count += c;
        }

        if let Some(c) = dict_count.get(&i_ai) {
            dict_count.insert(i_ai, c + 1);
        } else {
            dict_count.insert(i_ai, 1);
        }
    }

    println!("{count}");
}

// 安直に (stay symple)やったけどダメだった
// fn main() {
//     input! {
//         n: usize,
//         a: [i32; n]
//     }
//
//     let mut count: usize = 0;
//
//     let mut distances: Vec<i32> = vec![];
//
//     for i in 0..n {
//         distances.iter_mut().for_each(|x| *x += 1);
//         distances.push(-a[i]);
//
//         // println!("{distances:?}");
//         count += distances.iter().filter(|&&x| x == a[i]).count();
//     }
//
//     println!("{count}");
// }

// fn main() {
//     input! {
//         n: usize,
//         a: [usize; n]
//     }
//
//     let mut count = 0;
//
//     for i in 0..n {
//         if a[i] < n {
//             for j in a[i]..n {
//                 if j - i == a[i] + a[j] {
//                     count += 1;
//                 }
//             }
//         }
//     }
//
//     println!("{count}");
// }
