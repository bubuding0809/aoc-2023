use std::fmt::Error;
fn main() {
    let input = include_str!("./input-01.txt");
    println!("Part 2: {}", p2(input).unwrap());
}

fn p2(input: &str) -> Result<String, Error> {
    let res: u32 = input
        .lines()
        .map(|line| {
            let dices = line.split(":").last().unwrap();

            let (mut r, mut g, mut b) = (0, 0, 0);

            dices.trim().split(";").for_each(|subset| {
                subset.trim().split(",").for_each(|set| {
                    let mut it = set.trim().split_whitespace();
                    let count: u32 = it.next().unwrap_or("0").parse().unwrap_or(0);
                    let color = it.next().expect("Should be color");

                    match color {
                        "red" => r = r.max(count),
                        "green" => g = g.max(count),
                        "blue" => b = b.max(count),
                        _ => (),
                    };
                })
            });

            r * g * b
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
        assert_eq!(p2(test_input)?, "2286");
        Ok(())
    }
}
