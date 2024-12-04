use std::fs::File;
use std::io::{self, BufRead};

fn parse() -> Vec<Vec<char>> {
    let file = File::open("src/p04/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut grid = vec![];
    for line in reader.lines() {
        let row = line.unwrap().chars().collect();
        grid.push(row)
    }
    grid
}

fn solve(grid: Vec<Vec<char>>) {
    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    let dx: Vec<i32> = vec![-1, -1, 1, 1];
    let dy: Vec<i32> = vec![1, -1, -1, 1];
    let mut count = 0;
    for oy in 1..n - 1 {
        for ox in 1..m - 1 {
            if grid[oy as usize][ox as usize] != 'A' {
                continue;
            }
            let mut s_count = 0;
            let mut m_count = 0;
            for i in 0..dx.len() {
                let cy = (oy + dy[i]) as usize;
                let cx = (ox + dx[i]) as usize;
                if grid[cy][cx] == 'S' {
                    s_count += 1;
                }
                if grid[cy][cx] == 'M' {
                    m_count += 1;
                }
            }
            // `alt_match` is used to remove the following case as a match.
            // Specifically, we can't be an X-MAS if the top left and bottom right
            // corners match.
            // M.S
            // .A.
            // S.M
            let alt_match = grid[(oy - 1) as usize][(ox - 1) as usize]
                == grid[(oy + 1) as usize][(ox + 1) as usize];
            if s_count == 2 && m_count == 2 && !alt_match {
                count += 1;
            }
        }
    }
    println!("{count}");
}

pub fn run() {
    let grid = parse();
    solve(grid);
}
