use super::parser;

pub fn solve() {
    let (mut v1, mut v2) = parser::parse_file();

    v1.sort();
    v2.sort();
    let diff = v1
        .into_iter()
        .zip(v2.into_iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();
    println!("{diff}");
}
