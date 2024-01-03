use std::{collections::HashSet, fmt::Display};

fn main() {
    let input = include_str!("./input-01.txt");
    let output = p1(input);
    println!("Solution for part-01: {}", output)
}

struct Universe {
    map_grid: Vec<Vec<Node>>,
}

impl Universe {
    fn expand_rows(self) -> Self {
        let map_grid = self
            .map_grid
            .into_iter()
            .fold(Vec::new(), |mut universe, row| {
                // Expand universe if row does not contain any galaxies
                if !row.iter().any(|node| matches!(node, Node::Galaxy)) {
                    universe.push(row.clone())
                }

                universe.push(row);
                universe
            });

        Universe { map_grid }
    }

    fn expand_cols(self) -> Self {
        let cols = (0..self.map_grid[0].len()).fold(HashSet::new(), |mut cols, j| {
            if !(0..self.map_grid.len()).any(|i| {
                let node = &self.map_grid[i][j];
                matches!(node, Node::Galaxy)
            }) {
                cols.insert(j);
            }
            cols
        });

        let map_grid = self
            .map_grid
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .enumerate()
                    .fold(Vec::new(), |mut row, (j, node)| {
                        if cols.contains(&j) {
                            row.push(node.clone())
                        };

                        row.push(node.clone());
                        row
                    })
            })
            .collect();

        Universe { map_grid }
    }

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

fn shortest_path_length(from: (usize, usize), to: (usize, usize)) -> u32 {
    let vertical_len = from.0.abs_diff(to.0);
    let horizontal_len = from.1.abs_diff(to.1);

    (vertical_len + horizontal_len) as u32
}

fn p1(input: &str) -> String {
    let universe = parse_input(input);

    // Expand universe
    let universe = universe.expand_rows().expand_cols();

    // Find pairs of galaxies to calculate distance between each pair
    let galaxy_coords = universe.get_galaxy_coords();
    let pairs = find_pairs(galaxy_coords);

    // Get sum of all the shortest path length between the galaxies
    let res = pairs
        .iter()
        .map(|(from, to)| {
            let len = shortest_path_length(*from, *to);
            println!("{:?} -> {:?} found {} len", from, to, len);
            len
        })
        .sum::<u32>();

    res.to_string()
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
        assert_eq!(test_output, "374");
        Ok(())
    }

    #[test]
    fn test_get_galaxies() -> Result<(), Error> {
        let test_input = include_str!("./test-input-01.txt");
        let universe = parse_input(test_input);
        let universe = universe.expand_rows().expand_cols();

        let coords = universe.get_galaxy_coords();

        assert_eq!(
            coords,
            vec![
                (0, 4),
                (1, 9),
                (2, 0),
                (5, 8),
                (6, 1),
                (7, 12),
                (10, 9),
                (11, 0),
                (11, 5),
            ]
        );

        Ok(())
    }

    #[test]
    fn test_get_pairs() -> Result<(), Error> {
        let galaxies = vec![
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (7, 7),
            (8, 8),
            (9, 9),
        ];
        let pairs = find_pairs(galaxies);
        assert_eq!(pairs.len(), 36);

        Ok(())
    }

    #[test]
    fn test_shortest_path_length() -> Result<(), Error> {
        assert_eq!(shortest_path_length((0, 4), (2, 0)), 6);
        assert_eq!(shortest_path_length((0, 4), (6, 1)), 9);
        Ok(())
    }
}
