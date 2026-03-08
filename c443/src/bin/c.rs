use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
    }

    if n == 0 {
        println!("{t}");
        return;
    }

    input! {
        a: [usize; n]
    }

    // １回目のcloseを入れておく必要がある
    let mut recent_close = a[0];
    let mut closed_time: usize = 100;

    for ai in a {
        // 最近閉じた時より、100秒経過してからまた後ろ通ったら、また１００秒閉じる
        if (ai - recent_close) >= 100 {
            recent_close = ai;

            // 終業時間近くなければ+100
            if (t - recent_close) < 100 {
                closed_time += t - recent_close;
                break;
            } else {
                closed_time += 100;
            }
        }
    }

    println!("{}", t - closed_time);
}
