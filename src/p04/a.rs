use super::parser;

pub fn solve() {
    let grid = parser::parse_file();

    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    let dx: Vec<i32> = vec![1, 1, 1, 0, -1, -1, -1, 0];
    let dy: Vec<i32> = vec![-1, 0, 1, 1, 1, 0, -1, -1];
    const SEARCH_WORD: &str = "XMAS";
    let mut count = 0;
    for oy in 0..n {
        for ox in 0..m {
            for i in 0..dx.len() {
                let mut good = true;
                let mut cy: i32 = oy as i32;
                let mut cx: i32 = ox as i32;
                for j in 0..SEARCH_WORD.len() {
                    if cy < 0 || cy >= n || cx < 0 || cx >= m {
                        good = false;
                        break;
                    }
                    if grid[cy as usize][cx as usize] != SEARCH_WORD.chars().nth(j).unwrap() {
                        good = false;
                        break;
                    }
                    cy += dy[i];
                    cx += dx[i];
                }
                if good {
                    count += 1;
                }
            }
        }
    }
    println!("{count}");
}
