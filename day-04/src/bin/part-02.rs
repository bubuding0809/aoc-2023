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
    let card_winnings: Vec<usize> = cards_it
        .map(|(win, own)| win.intersection(&own).count())
        .collect();

    let mut scatchcard_piles = vec![1; card_winnings.len()];

    card_winnings.iter().enumerate().for_each(|(i, count)| {
        let curr_instances = scatchcard_piles[i];
        scatchcard_piles
            .iter_mut()
            .skip(i + 1)
            .take(*count)
            .for_each(|pile| {
                *pile += curr_instances;
            });
    });

    scatchcard_piles.iter().sum::<usize>().to_string()
}

// 4
// 2
// 2
// 1
// 0
// 0

//1
//2
//2
//2
//2
//1

#[cfg(test)]
mod tests {
    use std::fmt::Error;

    use super::*;

    #[test]
    fn shit_works() -> Result<(), Error> {
        let test_input = include_str!("./test-input-02.txt");
        let test_output = p1(test_input);
        println!("p1 test: {}", test_output);
        assert_eq!(test_output, "30");
        Ok(())
    }
}
