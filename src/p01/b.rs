use super::parser;

pub fn solve() {
    let (v1, v2) = parser::parse();
    let mut total = 0;
    for &a in v1.iter() {
        let mut count = 0;
        for &b in v2.iter() {
            if a == b {
                count += 1;
            }
        }
        total += a * count;
    }
    println!("{total}");
}
