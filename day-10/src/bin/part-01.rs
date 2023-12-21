use std::ops::ControlFlow;

fn main() {
    let input = include_str!("./input-01.txt");
    let output = p1(input);
    println!("Solution for part-01: {}", output)
}

#[derive(Debug)]
enum Tile {
    Start(Vec<(i32, i32)>),
    Connector(Vec<(i32, i32)>),
    Ground,
}

impl Tile {
    pub fn deltas(&self) -> Vec<(i32, i32)> {
        match self {
            Tile::Start(deltas) => deltas.clone(),
            Tile::Connector(deltas) => deltas.clone(),
            Tile::Ground => Vec::new(),
        }
    }
}

fn p1(input: &str) -> String {
    let grid = parse_input(input);
    let start = get_start_coords(&grid);
    let res = traverse_loop(start, &grid);
    res.to_string()
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => Tile::Start(vec![(1, 0), (0, 1), (-1, 0), (0, -1)]),
                    '|' => Tile::Connector(vec![(1, 0), (-1, 0)]),
                    '-' => Tile::Connector(vec![(0, -1), (0, 1)]),
                    'L' => Tile::Connector(vec![(-1, 0), (0, 1)]),
                    'J' => Tile::Connector(vec![(-1, 0), (0, -1)]),
                    '7' => Tile::Connector(vec![(1, 0), (0, -1)]),
                    'F' => Tile::Connector(vec![(1, 0), (0, 1)]),
                    _ => Tile::Ground,
                })
                .collect()
        })
        .collect()
}

fn get_start_coords(grid: &[Vec<Tile>]) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if let Tile::Start(_) = grid[i][j] {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn traverse_loop(start: (usize, usize), grid: &[Vec<Tile>]) -> u32 {
    let start_tile = &grid[start.0][start.1];

    // Find the valid tile next to start
    let mut prev = start;
    let (dy, dx) = start_tile
        .deltas()
        .into_iter()
        .find(|(dy, dx)| {
            // Get neighbour coords
            let (r, c) = (start.0 as i32 + dy, start.1 as i32 + dx);

            // Check if coordinates are out of range
            if r < 0 || (r as usize) >= grid.len() || c < 0 || (c as usize) >= grid[0].len() {
                return false;
            }

            // Check if neighbour is a valid destination
            let next_tile = &grid[r as usize][c as usize];
            next_tile.deltas().iter().any(|(dy, dx)| {
                let (k, l) = (r + dy, c + dx);
                start.0 as i32 == k && start.1 as i32 == l
            })
        })
        .unwrap();
    let mut curr = (
        (start.0 as i32 + dy) as usize,
        (start.1 as i32 + dx) as usize,
    );

    // Traverse loop to find steps to return to starting point

    let step = (1..).try_for_each(|s| {
        (prev, curr) = next_tile(prev, curr, grid);
        if curr == start {
            ControlFlow::Break(s + 1)
        } else {
            ControlFlow::Continue(())
        }
    });

    match step {
        ControlFlow::Break(s) => s / 2,
        _ => 0,
    }
}

fn next_tile(
    prev: (usize, usize),
    curr: (usize, usize),
    grid: &[Vec<Tile>],
) -> ((usize, usize), (usize, usize)) {
    // Find neighbour to traverse to, make sure not go away from origin
    let (dy, dx) = &grid[curr.0][curr.1]
        .deltas()
        .into_iter()
        .find(|(dy, dx)| {
            let (r, c) = (curr.0 as i32 + dy, curr.1 as i32 + dx);
            prev.0 != r.try_into().unwrap() || prev.1 != c.try_into().unwrap()
        })
        .unwrap();
    let to = ((curr.0 as i32 + dy) as usize, (curr.1 as i32 + dx) as usize);

    (curr, to)
}

#[cfg(test)]
mod tests {
    use std::fmt::Error;

    use super::*;

    #[test]
    fn shit_works() -> Result<(), Error> {
        let test_input = include_str!("./test-input-01.txt");
        let test_output = p1(test_input);
        println!("p1 test: {}", test_output);
        assert_eq!(test_output, "8");
        Ok(())
    }
}
