use super::parser;

pub fn solve() {
    let reports = parser::parse();
    let mut count = 0;
    for report in reports {
        let increasing = report.last().unwrap() > report.first().unwrap();
        let mut safe = true;
        for i in 0..(report.len() - 1) {
            if increasing && report[i] > report[i + 1] {
                safe = false;
                break;
            }
            if !increasing && report[i] < report[i + 1] {
                safe = false;
                break;
            }
            let diff = (report[i] - report[i + 1]).abs();
            if diff < 1 || diff > 3 {
                safe = false;
                break;
            }
        }
        if safe {
            count += 1;
        }
    }
    println!("{count}");
}
