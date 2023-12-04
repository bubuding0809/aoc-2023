fn main() {
    let input = include_str!("./input.txt");
    let output = p1(input);
    println!("Solution for part-01: {}", output)
}

fn p1(input: &str) -> String {
    let mut grid = process_input(input);

    for (i, row) in grid.iter().enumerate() {
        println!(
            "{} {} {}",
            i + 1,
            row.len(),
            row.iter().map(|x| x.0).collect::<String>()
        );
    }

    // Stamp all valid characters with true
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if is_symbol(&grid[i][j].0) {
                search_adj(i, j, &mut grid)
            }
        }
    }

    // Calculates the sum of valid numbers in a grid
    let res: u32 = grid
        .iter()
        .flat_map(|row| {
            row.iter()
                .fold(vec![String::new()], |mut acc, (c, valid)| {
                    if *valid {
                        if let Some(last) = acc.last_mut() {
                            *last = format!("{last}{c}");
                        }
                        acc
                    } else {
                        acc.push(String::new());
                        acc
                    }
                })
                .iter()
                .map(|x| x.parse::<u32>().unwrap_or(0))
                .collect::<Vec<u32>>()
        })
        .sum();

    res.to_string()
}

fn process_input(input: &str) -> Vec<Vec<(char, bool)>> {
    input
        .split("\n")
        .map(|line| line.chars().map(|c| (c, false)).collect())
        .collect()
}

fn is_symbol(c: &char) -> bool {
    !c.is_digit(10) && *c != '.'
}

fn search_adj(i: usize, j: usize, grid: &mut Vec<Vec<(char, bool)>>) {
    let adj_offsets: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dy, dx) in adj_offsets {
        let r: i32 = (i as i32) + dy;
        let c: i32 = (j as i32) + dx;

        // Ensure row and col indexes are within range
        if r < 0 || r >= (grid.len() as i32) || c < 0 || c >= (grid[0].len() as i32) {
            continue;
        };

        // Coerce indexes back to usize
        let r: usize = r.try_into().expect("row index should be valid usize");
        let c: usize = c.try_into().expect("col index  should be valid usize");

        // Only continue if char is a digit and is not visited
        if !grid[r][c].0.is_ascii_digit() || grid[r][c].1 {
            continue;
        }

        // Stamp current char as visited
        grid[r][c].1 = true;

        // Expand towards left to search for digits
        scan_left(&mut grid[r], c);

        // Expand towards right to search for digits
        scan_right(&mut grid[r], c);
    }
}

fn scan_left(row: &mut Vec<(char, bool)>, index: usize) {
    for i in (0..index).rev() {
        if row[i].0.is_ascii_digit() {
            row[i].1 = true;
        } else {
            break;
        }
    }
}

fn scan_right(row: &mut Vec<(char, bool)>, index: usize) {
    for i in index + 1..row.len() {
        if row[i].0.is_ascii_digit() {
            row[i].1 = true;
        } else {
            break;
        }
    }
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
        assert_eq!(test_output, "4361");
        Ok(())
    }
}
