use std::{collections::HashMap, fmt::Error};
fn main() {
    let input = include_str!("./input-02.txt");
    println!("Part 2: {}", p1(input).unwrap());
}

fn p1(input: &str) -> Result<String, Error> {
    let digit_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let res = input
        .lines()
        .map(|line| {
            let mut digits = String::new();

            for i in 0..line.len() {
                for j in i..line.len() {
                    let curr = &line[i..=j];
                    if let Ok(d) = curr.parse::<u32>() {
                        digits.push_str(&d.to_string())
                    } else if digit_map.contains_key(curr) {
                        digits.push_str(digit_map.get(curr).unwrap())
                    }
                }
            }
            let first = digits.chars().next().expect("Should be first digit");
            let second = digits.chars().last().expect("Should be last digit");

            format!("{first}{second}").parse::<u32>().unwrap_or(0)
        })
        .sum::<u32>();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shit_works() -> Result<(), Error> {
        let test_input = include_str!("./test-input-02.txt");
        assert_eq!(p1(test_input)?, "281");
        Ok(())
    }
}
