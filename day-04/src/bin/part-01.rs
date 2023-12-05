use std::collections::HashSet;

fn main() {
    let input = include_str!("./input-01.txt");
    let output = p1(input);
    println!("Solution for part-01: {}", output)
}

fn p1(input: &str) -> String {
    let cards_it = input.lines().map(|line| -> (HashSet<u32>, HashSet<u32>) {
        // Splits the line by ':' and retrieves the last element, trims it, and then splits it by '|'.
        let mut sets = line.split(':').last().unwrap().trim().split('|');

        // Extract winning numbers (first ele) into an iterator
        let winning_nums = sets
            .next()
            .unwrap_or("")
            .trim()
            .split_ascii_whitespace()
            .filter_map(|n| n.parse::<u32>().ok());

        // Extract drawn numbers (second ele) into an iterator
        let drawn_nums = sets
            .next()
            .unwrap_or("")
            .trim()
            .split_ascii_whitespace()
            .filter_map(|n| n.parse::<u32>().ok());

        // Return winning and drawn numbers as a tuple of sets
        (
            HashSet::from_iter(winning_nums),
            HashSet::from_iter(drawn_nums),
        )
    });

    // Map through each card and calculate score based on the number of cards won
    let res: u32 = cards_it
        .map(|(win, own)| {
            let won = win.intersection(&own).count();
            match dbg!(won) {
                0 => 0,
                _ => 2_u32.pow(u32::try_from(dbg!(won).saturating_sub(1)).unwrap()),
            }
        })
        .sum();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use std::fmt::Error;

    use super::*;

    #[test]
    fn shit_works() -> Result<(), Error> {
        let test_input = include_str!("./test-input.txt");
        let test_output = p1(test_input);
        println!("p1 test: {}", test_output);
        assert_eq!(test_output, "13");
        Ok(())
    }
}
