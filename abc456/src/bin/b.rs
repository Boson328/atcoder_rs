use proconio::input;

fn main() {
    input! {
        sai1: [usize; 6],
        sai2: [usize; 6],
        sai3: [usize; 6]
    }

    let sai1_4 = sai1.iter().filter(|&&x| x == 4).count();
    let sai1_5 = sai1.iter().filter(|&&x| x == 5).count();
    let sai1_6 = sai1.iter().filter(|&&x| x == 6).count();

    let sai2_4 = sai2.iter().filter(|&&x| x == 4).count();
    let sai2_5 = sai2.iter().filter(|&&x| x == 5).count();
    let sai2_6 = sai2.iter().filter(|&&x| x == 6).count();

    let sai3_4 = sai3.iter().filter(|&&x| x == 4).count();
    let sai3_5 = sai3.iter().filter(|&&x| x == 5).count();
    let sai3_6 = sai3.iter().filter(|&&x| x == 6).count();

    let petterns = sai1_4 * (sai2_5 * sai3_6 + sai2_6 * sai3_5)
        + sai1_5 * (sai2_4 * sai3_6 + sai2_6 * sai3_4)
        + sai1_6 * (sai2_4 * sai3_5 + sai2_5 * sai3_4);

    let probably = petterns as f64 / (6 * 6 * 6) as f64;

    print!("{probably}");
}
