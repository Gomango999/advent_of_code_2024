use super::parser;

pub fn solve() {
    let grid = parser::parse_file();

    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    // (dy[i], dx[i]) indicates the 4 offsets of the 4 corners of the X-MAS from
    // the centre.
    let dx: Vec<i32> = vec![-1, -1, 1, 1];
    let dy: Vec<i32> = vec![1, -1, -1, 1];
    let mut count = 0;
    for oy in 1..n - 1 {
        for ox in 1..m - 1 {
            // Check each potential centre point of an "X-MAS", and skip it if
            // it's not an 'A'.
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
            let alt_match = grid[(oy - 1) as usize][(ox - 1) as usize]
                == grid[(oy + 1) as usize][(ox + 1) as usize];
            // `alt_match` is used to remove the following case as a match.
            // M.S
            // .A.
            // S.M
            // Specifically, in the case that we have two S's and two M's in the
            // corners we can't be an "X-MAS" if the top-left and bottom-right
            // corners match.
            if s_count == 2 && m_count == 2 && !alt_match {
                count += 1;
            }
        }
    }
    println!("{count}");
}
