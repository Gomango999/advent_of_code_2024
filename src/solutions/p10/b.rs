use super::parser;

pub fn solve() {
    let map = parser::parse();

    // dp[y][x] stores the number of trails we can get starting at (y,x)
    // It starts out at all 0s, and we process coordinates in decreasing order
    // of height, starting from 9.
    let n = map.len();
    let m = map[0].len();
    let mut dp = vec![vec![0; m]; n];

    for h in (0..=9).rev() {
        for y in 0..n {
            for x in 0..m {
                if map[y][x] != h {
                    continue;
                }
                if h == 9 {
                    dp[y][x] = 1;
                } else {
                    let adj = [(1, 0), (0, 1), (-1, 0), (0, -1)];
                    for offset in adj {
                        let ny = (y as i32) + offset.0;
                        let nx = (x as i32) + offset.1;
                        if !(0..n).contains(&(ny as usize)) || !(0..m).contains(&(nx as usize)) {
                            continue;
                        }
                        if map[ny as usize][nx as usize] == map[y][x] + 1 {
                            dp[y][x] += dp[ny as usize][nx as usize];
                        }
                    }
                }
            }
        }
    }

    let mut total = 0;
    for y in 0..n {
        for x in 0..m {
            if map[y][x] == 0 {
                total += dp[y][x];
            }
        }
    }
    println!("{total}")
}
