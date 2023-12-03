use std::fmt::Error;
fn main() {
    let input = include_str!("./input-01.txt");
    println!("Part 1: {}", p1(input).unwrap());
}

struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

const CUBE_CONFIG: Cubes = Cubes {
    red: 12,
    green: 13,
    blue: 14,
};

fn p1(input: &str) -> Result<String, Error> {
    let res: u32 = input
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            let dices = line.split(":").last().unwrap();

            let not_possible = dices.trim().split(";").any(|subset| {
                subset.trim().split(",").any(|cube| {
                    let mut it = cube.trim().split_whitespace();
                    let count: u32 = it.next().unwrap_or("0").parse().unwrap_or(0);
                    let color = it.next().expect("Should be color");

                    match color {
                        "red" => count > CUBE_CONFIG.red,
                        "green" => count > CUBE_CONFIG.green,
                        "blue" => count > CUBE_CONFIG.blue,
                        _ => true,
                    }
                })
            });

            match not_possible {
                true => None,
                false => Some((i + 1) as u32),
            }
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
        assert_eq!(p1(test_input)?, "8");
        Ok(())
    }
}
