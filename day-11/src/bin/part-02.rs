use std::{cmp, collections::HashSet, fmt::Display};

fn main() {
    let input = include_str!("./input-02.txt");
    let output = p2(input);
    println!("Solution for part-02: {}", output)
}

struct Universe {
    map_grid: Vec<Vec<Node>>,
}

impl Universe {
    fn get_galaxy_coords(&self) -> Vec<(usize, usize)> {
        self.map_grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .fold(vec![], |mut coords, (j, node)| {
                        if let Node::Galaxy = node {
                            coords.push((i, j))
                        }
                        coords
                    })
            })
            .collect()
    }

    /// Returns the indices of the rows that can be expanded in the universe.
    fn expandable_rows(&self) -> HashSet<usize> {
        self.map_grid
            .iter()
            .enumerate()
            .filter_map(|(i, row)| {
                if row.iter().any(|node| matches!(node, Node::Galaxy)) {
                    None
                } else {
                    Some(i)
                }
            })
            .collect()
    }

    /// Returns the indices of the columns that can be expanded in the universe.
    fn expandable_cols(&self) -> HashSet<usize> {
        (0..self.map_grid[0].len())
            .filter(|j| {
                !(0..self.map_grid.len()).any(|i| {
                    let node = &self.map_grid[i][*j];
                    matches!(node, Node::Galaxy)
                })
            })
            .collect()
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binding = self
            .map_grid
            .iter()
            .map(|row| -> String {
                row.iter()
                    .map(|node| match node {
                        Node::Galaxy => '#',
                        Node::Space => '.',
                    })
                    .collect::<String>()
                    + "\n"
            })
            .collect::<String>();
        let display = binding.trim();
        write!(f, "{}", display)
    }
}

#[derive(Clone)]
enum Node {
    Galaxy,
    Space,
}

fn parse_input(input: &str) -> Universe {
    let map_grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Node::Galaxy,
                    _ => Node::Space,
                })
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Vec<Node>>>();

    Universe { map_grid }
}

fn find_pairs(coords: Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
    coords
        .iter()
        .enumerate()
        .flat_map(|(i, from)| {
            coords
                .iter()
                .skip(i + 1)
                .map(|to| (*from, *to))
                .collect::<Vec<((usize, usize), (usize, usize))>>()
        })
        .collect()
}

fn shortest_path_length(
    from: (usize, usize),
    to: (usize, usize),
    expansion: u64,
    exp_rows: &HashSet<usize>,
    exp_cols: &HashSet<usize>,
) -> u64 {
    let mut vertical_len = from.0.abs_diff(to.0) as u64;
    let mut horizontal_len = from.1.abs_diff(to.1) as u64;

    let vertical_addition =
        (from.0..to.0).filter(|i| exp_rows.contains(i)).count() as u64 * (expansion - 1);

    let col_range = match from.1.cmp(&to.1) {
        cmp::Ordering::Less => from.1..to.1,
        _ => to.1..from.1,
    };
    let horizontal_addition =
        col_range.filter(|j| exp_cols.contains(j)).count() as u64 * (expansion - 1);

    vertical_len += vertical_addition;
    horizontal_len += horizontal_addition;

    vertical_len + horizontal_len
}

fn p2(input: &str) -> String {
    let universe = parse_input(input);
    let coords = universe.get_galaxy_coords();
    let pairs = find_pairs(coords);
    let expandable_rows = dbg!(universe.expandable_rows());
    let expandable_cols = dbg!(universe.expandable_cols());

    // Get sum of all the shortest path length between the galaxies
    let res = pairs
        .iter()
        .map(|(from, to)| {
            let len = shortest_path_length(*from, *to, 1000000, &expandable_rows, &expandable_cols);
            println!("{:?} -> {:?} found {} len", from, to, len);
            len
        })
        .sum::<u64>();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use std::io::Error;

    use super::*;

    #[test]
    fn shit_works() -> Result<(), Error> {
        let test_input = include_str!("./test-input-02.txt");
        let test_output = p2(test_input);
        println!("p2 test: {}", test_output);
        assert_eq!(test_output, "374");
        Ok(())
    }

    #[test]
    fn test_expandable_rows_empty() {
        let universe = Universe {
            map_grid: vec![
                vec![Node::Space, Node::Space, Node::Space],
                vec![Node::Space, Node::Space, Node::Space],
                vec![Node::Space, Node::Space, Node::Space],
            ],
        };
        let expected = HashSet::from([0, 1, 2]);
        assert_eq!(universe.expandable_rows(), expected);
    }

    #[test]
    fn test_expandable_rows_full() {
        let universe = Universe {
            map_grid: vec![
                vec![Node::Galaxy, Node::Galaxy, Node::Galaxy],
                vec![Node::Galaxy, Node::Galaxy, Node::Galaxy],
                vec![Node::Galaxy, Node::Galaxy, Node::Galaxy],
            ],
        };
        let expected = HashSet::new();
        assert_eq!(universe.expandable_rows(), expected);
    }

    #[test]
    fn test_expandable_rows_mixed() {
        let universe = Universe {
            map_grid: vec![
                vec![Node::Space, Node::Galaxy, Node::Space],
                vec![Node::Galaxy, Node::Space, Node::Galaxy],
                vec![Node::Space, Node::Space, Node::Space],
            ],
        };
        let expected = HashSet::from([2]);
        assert_eq!(universe.expandable_rows(), expected);
    }

    #[test]
    fn test_expand_cols_empty() {
        let universe = Universe {
            map_grid: vec![
                vec![Node::Space, Node::Space, Node::Space],
                vec![Node::Space, Node::Space, Node::Space],
                vec![Node::Space, Node::Space, Node::Space],
            ],
        };

        let expected = HashSet::from([0, 1, 2]);
        assert_eq!(universe.expandable_cols(), expected);
    }

    #[test]
    fn test_shortest_path_length() {
        let from = (0, 0);
        let to = (2, 2);
        let expansion = 4;
        let exp_rows: HashSet<usize> = [1].iter().cloned().collect();
        let exp_cols: HashSet<usize> = [1].iter().cloned().collect();

        let result = shortest_path_length(from, to, expansion, &exp_rows, &exp_cols);
        assert_eq!(result, 10);
    }
}
