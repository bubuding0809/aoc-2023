fn main() {
    let input = include_str!("./input-02.txt");
    let output = p2(input);
    println!("Solution for part-02: {}", output)
}

fn p2(input: &str) -> String {
    let mut text_chunks_it = input.split("\n\n");

    // Extracts the seeds from the given text chunks and returns an iterator over them.
    let seeds_it: Vec<(&str, &str)> = text_chunks_it
        .next()
        .unwrap_or_default()
        .split(':')
        .last()
        .unwrap_or_default()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    // Maps the ranges of source and destination based on the given text chunks.
    // Returns an iterator of HashMaps where the keys are the source ranges and the values are the destination ranges.
    let mappings_it: Vec<Vec<(u64, u64, u64)>> = text_chunks_it
        .map(|chunk| {
            let range: Vec<(u64, u64, u64)> = chunk
                .lines()
                .skip(1)
                .map(|line| {
                    let mut ranges_it = line.trim().split_ascii_whitespace();
                    let to_u64 =
                        |s: Option<&str>| s.unwrap_or_default().parse::<u64>().unwrap_or(0);
                    let destination = to_u64(ranges_it.next());
                    let source = to_u64(ranges_it.next());
                    let length = to_u64(ranges_it.next());
                    (destination, source, length)
                })
                .collect();
            range
        })
        .collect();

    // Calculates the minimum value obtained by applying a series of mappings to a set of seeds.
    let res = seeds_it
        .iter()
        .filter_map(
            |(start, length)| match (start.parse::<u64>(), length.parse::<u64>()) {
                (Ok(s), Ok(l)) => Some((s, s + l)),
                _ => None,
            },
        )
        .flat_map(|(start, end)| (start..end))
        .map(|seed| {
            mappings_it.iter().fold(seed, |acc, curr| {
                curr.iter()
                    .find(|(_, src, len)| (*src..*src + *len).contains(&acc))
                    .map_or(acc, |(dest, src, _)| acc - src + dest)
            })
        })
        .min()
        .unwrap_or(0);

    res.to_string()
}

#[cfg(test)]
mod tests {
    use std::fmt::Error;

    use super::*;

    #[test]
    fn shit_works() -> Result<(), Error> {
        let test_input = include_str!("./test-input-01.txt");
        let test_output = p2(test_input);
        println!("p2 test: {}", test_output);
        assert_eq!(test_output, "46");
        Ok(())
    }
}
