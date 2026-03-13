use proconio::input;

// スタック構造は偉大
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    let mut puyo_stack: Vec<usize> = vec![];

    for ai in a {
        puyo_stack.push(ai);
        let length = puyo_stack.len();
        if length >= 4
            && ai == puyo_stack[length - 2]
            && ai == puyo_stack[length - 3]
            && ai == puyo_stack[length - 4]
        {
            puyo_stack.truncate(length - 4);
        }
    }

    let count = puyo_stack.len();
    println!("{count}");
}

// あんまり深く考えすぎた 普通にスタックで解ける
// fn puyo_rensa(puyo_len: &mut Vec<(usize, usize)>, index: usize) {
//     let length = puyo_len.len();
//     puyo_len[index].1 -= 4;
//
//     if puyo_len[index].1 == 0 {
//         puyo_len.remove(index);
//
//         if index == 0 || index == length - 1 {
//             return;
//         }
//
//         if puyo_len[index - 1].0 == puyo_len[index].0 {
//             puyo_len[index - 1].1 += puyo_len[index].1;
//             puyo_len.remove(index);
//
//             if puyo_len[index - 1].1 >= 4 {
//                 puyo_rensa(puyo_len, index - 1);
//             }
//         }
//     }
// }
//
// fn main() {
//     input! {
//         n: usize,
//         a: [usize;n]
//     }
//
//     //                     puyo   len
//     let mut puyo_len: Vec<(usize, usize)> = vec![];
//
//     for ai in a {
//         if puyo_len.is_empty() {
//             puyo_len.push((ai, 1));
//             continue;
//         }
//
//         let length = puyo_len.len();
//         let (puyo, len) = puyo_len[length - 1];
//
//         if puyo == ai {
//             puyo_len[length - 1] = (puyo, len + 1);
//         } else {
//             puyo_len.push((ai, 1));
//         }
//     }
//
//     let mut i = 0;
//
//     loop {
//         if puyo_len[i].1 >= 4 {
//             puyo_rensa(&mut puyo_len, i);
//
//             i = 0;
//             continue;
//         }
//
//         if i == puyo_len.len() - 1 {
//             break;
//         }
//
//         i += 1;
//     }
//
//     let sum = puyo_len.iter().fold(0, |s, (_, len)| s + len);
//     println!("{sum}");
// }
