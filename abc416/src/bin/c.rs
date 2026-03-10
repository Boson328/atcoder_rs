use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize, //数列の長さ
        x: Usize1, // IndexなのでUsize1
        s: [String; n]
    }

    let n_k = n.pow(k as u32);
    let mut dict: Vec<String> = vec![];
    let mut current_seq = vec![0; k];

    for _ in 0..n_k {
        for i in 0..k {
            if current_seq[i] != n {
                break;
            }
            current_seq[i] = 0;
            current_seq[i + 1] += 1;
        }

        let mut current_string = String::new();

        for i in 0..k {
            current_string += &s[current_seq[i]];
        }

        dict.push(current_string);

        current_seq[0] += 1;
    }

    dict.sort();

    println!("{}", dict[x]);
}

// 通らなかった。文字列の長さが違うため、a + ba   az + aとあった時、aza < abaと判定されてしまい間違い
// 文字列長が同じなら通った

// use proconio::{input, marker::Usize1};
//
// fn main() {
//     input! {
//         n: usize,
//         k: usize, //数列の長さ
//         x: Usize1, // IndexなのでUsize1
//         s: [String; n]
//     }
//
//     let mut s_sort = s.clone();
//     let mut pri: Vec<String> = vec![];
//
//     s_sort.sort();
//
//     // 進数変換でクリアする
//     // 0~n-1, 0~n-1 という数字の並びはn進数に近い
//     // 変換後の数をソートしたsに当てはめればいい
//     let mut converting = x;
//
//     for _ in 0..k {
//         // a回目の剰余は下からa個目の桁をあらわす
//         let digit = converting % n;
//         converting /= n;
//
//         // 下の桁から把握していくため、配列の先頭から追加
//         pri.insert(0, s_sort[digit].clone());
//     }
//
//     for p in pri {
//         print!("{p}");
//     }
//
//     println!();
// }
