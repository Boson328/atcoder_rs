use proconio::input;
use regex::Regex;

fn main() {
    input! {
        s: String
    }

    let re = Regex::new(r"(W+)A").unwrap();
    let s = re.replace_all(&s, |caps: &regex::Captures| {
        let w_count = caps[1].len();

        format!("A{}", "C".repeat(w_count))
    });

    println!("{s}");
}
