use std::fmt::Error;
fn main() {
    let input = include_str!("./input-01.txt");
    println!("Part 1: {}", p1(input).unwrap());
}

fn p1(input: &str) -> Result<String, Error> {
    let res: u32 = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|c| c.to_digit(10));

            let first = it.next().expect("first digit");
            match it.last() {
                Some(last) => format!("{first}{last}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .unwrap_or_default()
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
