use proconio::input;

fn main() {
    input! {
        h: u8,
        w: u8,
    }

    for _ in 0..w {
        print!("#");
    }

    println!("");

    for _ in 1..h - 1 {
        print!("#");
        for _ in 1..w - 1 {
            print!(".");
        }
        println!("#");
    }

    for _ in 0..w {
        print!("#");
    }
    println!("");
}
