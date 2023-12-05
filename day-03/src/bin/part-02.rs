use std::{collections::VecDeque, ops::ControlFlow};

fn main() {
    let input = include_str!("./input.txt");
    let output = p2(input);
    println!("Solution for part-01: {}", output)
}

fn p2(input: &str) -> String {
    let mut grid = process_input(input);
    let mut gears = Vec::new();

    // Stamp all valid characters with true
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '*' {
                gears.push(get_adj_nums(i, j, &mut grid));
            }
        }
    }

    // Calculates the sum of valid numbers in a grid
    let res: u32 = gears
        .iter()
        .inspect(|x| {
            dbg!(x);
        })
        .filter_map(|nums| match nums.len() {
            2 => Some(nums.iter().product::<u32>()),
            _ => None,
        })
        .sum();

    res.to_string()
}

fn process_input(input: &str) -> Vec<Vec<char>> {
    input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect()
}

fn get_adj_nums(i: usize, j: usize, grid: &mut Vec<Vec<char>>) -> Vec<u32> {
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

    adj_offsets
        .iter()
        .filter_map(|(dy, dx)| {
            let r: i32 = (i as i32) + dy;
            let c: i32 = (j as i32) + dx;

            // Ensure row and col indexes are within range
            if r < 0 || r >= (grid.len() as i32) || c < 0 || c >= (grid[0].len() as i32) {
                return None;
            }

            // Coerce indexes back to usize
            let r: usize = r.try_into().expect("row index should be valid usize");
            let c: usize = c.try_into().expect("col index  should be valid usize");

            // Only continue if char is a digit and is not visited
            if !grid[r][c].is_ascii_digit() || grid[r][c] == '#' {
                return None;
            }

            // Double-ended queue to store number
            let mut num_deque: VecDeque<char> = VecDeque::from([grid[r][c]]);
            // Expand towards left to search for digits
            scan_left(&mut grid[r], &mut num_deque, c);
            // Expand towards right to search for digits
            scan_right(&mut grid[r], &mut num_deque, c);
            num_deque.iter().collect::<String>().parse::<u32>().ok()
        })
        .collect::<Vec<u32>>()
}

fn scan_left(row: &mut [char], num: &mut VecDeque<char>, index: usize) {
    for i in (0..index).rev() {
        if row[i].is_ascii_digit() {
            num.push_front(row[i]);
            row[i] = '#';
        } else {
            break;
        }
    }
}

fn scan_right(row: &mut [char], num: &mut VecDeque<char>, index: usize) {
    row.iter_mut().skip(index + 1).try_for_each(|col| {
        if !col.is_ascii_digit() {
            ControlFlow::Break(())
        } else {
            num.push_back(*col);
            *col = '#';
            ControlFlow::Continue(())
        }
    });
}

#[cfg(test)]
mod tests {
    use std::fmt::Error;

    use super::*;

    #[test]
    fn shit_works() -> Result<(), Error> {
        let test_input = include_str!("./test-input-01.txt");
        let test_output = p2(test_input);
        println!("p2 test: {}", test_output);
        assert_eq!(test_output, "467835");
        Ok(())
    }
}
