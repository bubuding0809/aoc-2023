fn main() {
    let input = include_str!("./input-02.txt");
    let output = p2(input);
    println!("Solution for part-02: {}", output)
}

fn p2(input: &str) -> String {
    let parsed_input = parse_input(input);
    let res: i64 = parsed_input
        .iter()
        .map(|vals| {
            let mut stack = Vec::from([vals.clone()]);
            while stack.last().unwrap().iter().any(|val| *val != 0) {
                stack.push(get_diffs(stack.last().unwrap()));
            }
            get_extrapolated_val(&stack)
        })
        .sum();
    res.to_string()
}

fn get_extrapolated_val(stack: &[Vec<i64>]) -> i64 {
    stack
        .iter()
        .filter_map(|row| row.first())
        .rev()
        .copied()
        .reduce(|acc, curr| (curr - acc))
        .unwrap()
}

fn get_diffs(vals: &[i64]) -> Vec<i64> {
    let diffs = vals.iter().skip(1).fold(
        (vec![], vals.first().unwrap().to_owned()),
        |mut acc: (Vec<i64>, i64), curr| {
            acc.0.push(curr - acc.1);
            acc.1 = *curr;
            acc
        },
    );
    diffs.0
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|val| val.parse::<i64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i64>>>()
}

#[cfg(test)]
mod tests {
    use std::fmt::Error;

    use super::*;

    #[test]
    fn shit_works() -> Result<(), Error> {
        let test_input = include_str!("./test-input-02.txt");
        let test_output = p2(test_input);
        println!("p2 test: {}", test_output);
        assert_eq!(test_output, "2");
        Ok(())
    }
}
