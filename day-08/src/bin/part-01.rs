use std::{collections::HashMap, ops::ControlFlow};

fn main() {
    let input = include_str!("./input-01.txt");
    let output = p1(input);
    println!("Solution for part-01: {}", output)
}

fn parse_input(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
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

    (steps, map)
}

fn navigate_map(steps: &str, map: HashMap<&str, (&str, &str)>) -> String {
    let res = steps
        .chars()
        .cycle()
        .try_fold((0_u32, "AAA"), |(count, key), curr| {
            // Get l and r options based on current key
            let (l, r) = map.get(key).unwrap();

            // Find next key based on step
            let curr_key = match curr {
                'L' => l,
                'R' => r,
                _ => l,
            };

            // Break out if destination is reached
            if curr_key.eq(&"ZZZ") {
                ControlFlow::Break(count + 1)
            } else {
                ControlFlow::Continue((count + 1, *curr_key))
            }
        });

    if let ControlFlow::Break(count) = res {
        count.to_string()
    } else {
        panic!("SHIT HAPPENED")
    }
}

fn p1(input: &str) -> String {
    let (steps, map) = parse_input(input);
    navigate_map(steps, map)
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
        assert_eq!(test_output, "6");
        Ok(())
    }
}
