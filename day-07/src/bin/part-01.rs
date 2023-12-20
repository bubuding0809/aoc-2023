use std::{
    collections::{HashMap, VecDeque},
    ops::ControlFlow,
};

#[derive(Debug)]
struct Hand {
    cards: [String; 5],
    bid: u32,
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind(u32),
    FourOfAKind(u32),
    FullHouse(u32),
    ThreeOfaKind(u32),
    TwoPair(u32),
    OnePair(u32),
    HighCard(u32),
}

impl HandType {
    fn from(cards: &[String; 5]) -> Self {
        let freqs = cards.iter().fold(HashMap::new(), |mut acc, curr| {
            *acc.entry(curr).or_insert(0) += 1;
            acc
        });

        let max_freq_pair = freqs.iter().max_by_key(|(_, v)| (*v)).unwrap();
        let max_count = if *max_freq_pair.0 == &"J".to_string() {
            max_freq_pair.1 + freqs.get(&"J".to_string()).unwrap()
        } else {
            *max_freq_pair.1
        };

        match max_count {
            // Check for Five of a kind and Four of a kind
            5 => HandType::FiveOfAKind(6),
            4 => HandType::FourOfAKind(5),

            // Check for Full house or Three of a Kind
            3 => {
                if freqs.iter().any(|(_, v)| *v == 2) {
                    HandType::FullHouse(4)
                } else {
                    HandType::ThreeOfaKind(3)
                }
            }

            // Check for Two Pair or One Pair
            2 => {
                if freqs.iter().filter(|(_, v)| **v == 2).count().eq(&2_usize) {
                    HandType::TwoPair(2)
                } else {
                    HandType::OnePair(1)
                }
            }

            // Check for High card or invalid combinationss
            1 => HandType::HighCard(0),
            _ => HandType::HighCard(0),
        }
    }
}

fn main() {
    let input = include_str!("./input-01.txt");
    let output = p1(input);
    println!("Solution for part-01: {}", output)
}

fn p1(input: &str) -> String {
    let hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let mut pair_it = line.split_ascii_whitespace();
            let cards: [String; 5] = pair_it
                .next()
                .unwrap()
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .try_into()
                .unwrap();
            let bid: u32 = pair_it.next().unwrap().parse().unwrap();

            Hand { cards, bid }
        })
        .collect();

    let mut type_groups = hands.into_iter().fold(
        (0..7).map(|_i| Vec::new()).collect::<Vec<Vec<Hand>>>(),
        |mut acc, curr| {
            match HandType::from(&curr.cards) {
                HandType::FiveOfAKind(i) => acc[i as usize].push(curr),
                HandType::FourOfAKind(i) => acc[i as usize].push(curr),
                HandType::FullHouse(i) => acc[i as usize].push(curr),
                HandType::ThreeOfaKind(i) => acc[i as usize].push(curr),
                HandType::TwoPair(i) => acc[i as usize].push(curr),
                HandType::OnePair(i) => acc[i as usize].push(curr),
                HandType::HighCard(i) => acc[i as usize].push(curr),
            };
            acc
        },
    );

    let card_vals: HashMap<String, u32> = HashMap::from([
        (String::from("A"), 13),
        (String::from("K"), 12),
        (String::from("Q"), 11),
        (String::from("T"), 10),
        (String::from("J"), 1),
    ]);

    let res: u32 = type_groups
        .iter_mut()
        .filter(|hands| !hands.is_empty())
        .flat_map(|hands| {
            let mut ranges = VecDeque::from([(0_usize, hands.len() - 1)]);

            (0..5).try_for_each(|i| {
                if ranges.is_empty() {
                    return ControlFlow::Break(());
                }

                (0..ranges.len()).for_each(|_| {
                    let range = ranges.pop_front().unwrap();

                    hands[range.0..=range.1].sort_by_key(|h| match h.cards[i].parse::<u32>() {
                        Ok(v) => v,
                        Err(_) => *card_vals.get(&h.cards[i]).unwrap(),
                    });

                    // Gather to be sorted ranges
                    let binding = hands[range.0..=range.1].iter().enumerate().fold(
                        HashMap::new(),
                        |mut acc: HashMap<String, Vec<usize>>, (j, curr)| {
                            (*acc.entry(curr.cards[i].clone()).or_default()).push(j + range.0);
                            acc
                        },
                    );

                    let mut to_be_sorted: VecDeque<(usize, usize)> = binding
                        .into_iter()
                        .filter_map(|(_, v)| match v.len().gt(&1_usize) {
                            true => Some((*v.first().unwrap(), *v.last().unwrap())),
                            false => None,
                        })
                        .collect();

                    ranges.append(&mut to_be_sorted);
                });
                ControlFlow::Continue(())
            });

            // Return the sorted list of bids
            hands.iter().map(|hand| hand.bid).collect::<Vec<u32>>()
        })
        .enumerate()
        .map(|(i, bid)| ((i as u32) + 1) * bid)
        .sum();

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
        assert_eq!(test_output, "6440");
        Ok(())
    }

    #[test]
    fn shit_works_on_subset() -> Result<(), Error> {
        let test_input = include_str!("./subset_5.txt");
        let test_output = p1(test_input);
        println!("subset 5 test: {}", test_output);
        Ok(())
    }
}
