use std::collections::VecDeque;

use proconio::input;

// 特に工夫しなくても動く

fn main() {
    input! {
        t: usize
    }

    let mut pri = vec![];

    for _ in 0..t {
        input! {
            n: usize,
            d: usize,
            a: [i32; n],
            b: [i32; n]
        }

        let mut egg_list: VecDeque<(i32, i32)> = VecDeque::new();

        for i in 0..n {
            egg_list.push_back((i as i32, a[i]));

            let mut need = b[i];
            while need > 0 {
                let new_egg = egg_list[0].1;

                if new_egg <= need {
                    need -= new_egg;
                    egg_list.pop_front();
                } else {
                    egg_list[0].1 -= need;
                    need -= new_egg;
                }
            }

            if !egg_list.is_empty() && i >= d && (i - d) as i32 == egg_list[0].0 {
                egg_list.pop_front();
            }
        }

        pri.push(egg_list.iter().fold(0, |x, (_day, count)| x + count));
    }

    for p in pri {
        println!("{p}");
    }
}
