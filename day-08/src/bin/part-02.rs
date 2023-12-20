use num::integer::lcm;
use std::{collections::HashMap, ops::ControlFlow};

fn main() {
    let input = include_str!("./input-02.txt");
    let output = p1(input);
    println!("Solution for part-02: {}", output)
}

fn parse_input(input: &str) -> (&str, HashMap<&str, (&str, &str)>, Vec<&str>) {
    let mut chunks_it = input.split("\n\n");

    let steps = chunks_it.next().unwrap_or_default();

    let map = chunks_it.next().unwrap_or_default().lines().fold(
        HashMap::new(),
        |mut acc: HashMap<&str, (&str, &str)>, curr| {
            let mut it = curr.split('=');

            let key = it.next().unwrap_or_default().trim();

            let value = it.next().unwrap_or_default().trim();
            let value = &value[1..value.len() - 1];

            let l = value.split(',').next().unwrap().trim();
            let r = value.split(',').next_back().unwrap().trim();

            acc.insert(key, (l, r));

            acc
        },
    );

    let starts: Vec<&str> = map
        .iter()
        .filter_map(|(k, _v)| match k.ends_with('A') {
            true => Some(*k),
            false => None,
        })
        .collect();

    (steps, map, starts)
}

fn find_steps_to_z(start_node: &str, steps: &str, map: &HashMap<&str, (&str, &str)>) -> u128 {
    let count = steps
        .chars()
        .cycle()
        .try_fold((0_u128, start_node), |(count, node), direction| {
            let (l, r) = map.get(node).unwrap();
            let next_node = match direction {
                'L' => l,
                _ => r,
            };

            if next_node.ends_with('Z') {
                ControlFlow::Break(count + 1_u128)
            } else {
                ControlFlow::Continue((count + 1_u128, *next_node))
            }
        });

    match count {
        ControlFlow::Break(count) => count,
        _ => panic!("Should be a break instead"),
    }
}

fn p1(input: &str) -> String {
    let (steps, map, starts) = parse_input(input);
    let all_steps_taken: Vec<u128> = starts
        .into_iter()
        .map(|start_node| find_steps_to_z(start_node, steps, &map))
        .collect();

    lcm_multiple(&all_steps_taken).to_string()
}

fn lcm_multiple(nums: &[u128]) -> u128 {
    nums.iter().fold(1, |acc, curr| lcm(acc, *curr))
}

#[cfg(test)]
mod tests {
    use std::fmt::Error;

    use super::*;

    #[test]
    fn shit_works() -> Result<(), Error> {
        let test_input = include_str!("./test-input-02.txt");
        let test_output = p1(test_input);
        println!("p2 test: {}", test_output);
        assert_eq!(test_output, "6");
        Ok(())
    }

    #[test]
    fn test_input_parsing() -> Result<(), Error> {
        let test_input = include_str!("./test-input-02.txt");
        let (_steps, _map, starts) = parse_input(test_input);
        dbg!(_steps, _map, starts);
        Ok(())
    }

    #[test]
    fn test_lcm_multiple() -> Result<(), Error> {
        let test_input = vec![4, 5, 3];
        assert_eq!(lcm_multiple(&test_input), 60);
        Ok(())
    }
}
