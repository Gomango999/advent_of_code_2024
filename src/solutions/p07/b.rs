use super::parser;
use super::parser::Calibration;

fn int_concat(a: i64, b: i64) -> i64 {
    let str = a.to_string() + &b.to_string();
    str.parse::<i64>().unwrap()
}

// is_solvable_rec(target, current, nums) returns a boolean indicating whether
// it is possible to make the target from the numbers [current, ..nums].
fn is_solvable_rec(target: i64, current: i64, nums: &[i64]) -> bool {
    if nums.len() == 0 {
        return current == target;
    }

    let possible_with_add = is_solvable_rec(target, current + nums[0], &nums[1..]);
    let possible_with_mul = is_solvable_rec(target, current * nums[0], &nums[1..]);
    let possible_with_concat = is_solvable_rec(target, int_concat(current, nums[0]), &nums[1..]);
    possible_with_add || possible_with_mul || possible_with_concat
}

fn is_solvable(calibration: &Calibration) -> bool {
    is_solvable_rec(
        calibration.target,
        calibration.nums[0],
        &calibration.nums[1..],
    )
}

pub fn solve() {
    let calibrations = parser::parse();

    let mut total = 0;
    for calibration in calibrations {
        if is_solvable(&calibration) {
            total += calibration.target
        }
    }
    println!("{total}");
}
