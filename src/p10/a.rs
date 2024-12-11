use super::parser;

/// `dfs` returns the number of unique 9's it has seen from valid trails starting
/// from (y,x)
fn dfs(y: usize, x: usize, map: &Vec<Vec<u32>>, seen: &mut Vec<Vec<bool>>) -> i32 {
    if seen[y][x] {
        return 0;
    }
    seen[y][x] = true;

    if map[y][x] == 9 {
        return 1;
    }

    let n = seen.len();
    let m = seen[0].len();
    let adj = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut sum = 0;
    for offset in adj {
        let ny = (y as i32) + offset.0;
        let nx = (x as i32) + offset.1;
        if !(0..n).contains(&(ny as usize)) || !(0..m).contains(&(nx as usize)) {
            continue;
        }
        let ny = ny as usize;
        let nx = nx as usize;
        if map[ny as usize][nx as usize] == map[y][x] + 1 {
            sum += dfs(ny, nx, map, seen);
        }
    }
    sum
}

pub fn solve() {
    let map = parser::parse();

    // dp[y][x] stores the number of trails we can get starting at (y,x)
    let n = map.len();
    let m = map[0].len();

    let mut total = 0;
    for y in 0..n {
        for x in 0..m {
            if map[y][x] == 0 {
                let mut seen = vec![vec![false; m]; n];
                let trail_heads = dfs(y, x, &map, &mut seen);
                total += trail_heads
            }
        }
    }
    println!("{total}")
}
