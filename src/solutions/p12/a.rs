use super::parser;

struct Plot {
    area: u64,
    perimeter: u64,
}

fn is_inside(y: i32, x: i32, garden: &Vec<Vec<char>>) -> bool {
    let n = garden.len() as i32;
    let m = garden[0].len() as i32;
    (0..n).contains(&y) && (0..m).contains(&x)
}

/// For a given (x,y), dfs will return the area and perimeter of that plot.
fn find_plot(y: usize, x: usize, garden: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>) -> Plot {
    if seen[y][x] {
        return Plot {
            area: 0,
            perimeter: 0,
        };
    }
    seen[y][x] = true;

    let mut plot = Plot {
        area: 1,
        perimeter: 0,
    };
    let adj = [(1, 0), (0, 1), (0, -1), (-1, 0)];
    for offset in adj {
        let ny = y as i32 + offset.0;
        let nx = x as i32 + offset.1;
        if !is_inside(ny, nx, garden) {
            plot.perimeter += 1;
            continue;
        }
        let ny = ny as usize;
        let nx = nx as usize;
        if garden[ny][nx] != garden[y][x] {
            plot.perimeter += 1;
        } else {
            let adj_plot = find_plot(ny, nx, garden, seen);
            plot.area += adj_plot.area;
            plot.perimeter += adj_plot.perimeter;
        }
    }
    plot
}

pub fn solve() {
    let garden = parser::parse();

    let n = garden.len();
    let m = garden[0].len();

    let mut seen = vec![vec![false; m]; n];

    let mut total: u64 = 0;
    for y in 0..n {
        for x in 0..m {
            let plot = find_plot(y, x, &garden, &mut seen);
            total += plot.area * plot.perimeter;
        }
    }
    println!("{total}")
}
