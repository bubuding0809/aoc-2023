use std::fmt::{format, Error};

fn main() {
    let input = include_str!("./input-01.txt");
    println!("Part 1: {}", p1(input).unwrap());
}

fn p1(input: &str) -> Result<String, Error> {
    let res: u32 = input
        .lines()
        .map(|line| {
            let digits = line.chars().fold(String::new(), |acc, curr| {
                if curr.is_ascii_digit() {
                    format!("{acc}{curr}")
                } else {
                    acc
                }
            });

            let first_digit = digits.chars().next().expect("Should be first digit");
            let last_digit = digits.chars().last().expect("Should be last digit");
            format!("{}{}", first_digit, last_digit)
                .parse::<u32>()
                .unwrap()
        })
        .sum();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shit_works() -> Result<(), Error> {
        let test_input = include_str!("./test-input-01.txt");
        assert_eq!(p1(test_input)?, "142");
        Ok(())
    }
}
