fn main() {
    let input = include_str!("./input-02.txt");
    let output = p1(input);
    println!("Solution for part-01: {}", output)
}

fn p1(input: &str) -> String {
    let to_vec_f64 = |s: Option<&str>| {
        s.unwrap()
            .split(':')
            .last()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .fold(String::new(), |acc, curr| acc + curr)
            .parse::<f64>()
            .unwrap()
    };

    let mut lines_it = input.lines();
    let time = to_vec_f64(lines_it.next());
    let dist = to_vec_f64(lines_it.next());
    dbg!(&time, &dist);

    let (s, e) = get_x_limits(time, -1_f64, -dist);
    from_x_limits(s, e).to_string()
}

fn get_x_limits(b: f64, a: f64, c: f64) -> (f64, f64) {
    let x1 = (-b + (f64::sqrt(f64::powi(b, 2) - 4.0 * a * c))) / (2.0 * a);
    let x2 = (-b - (f64::sqrt(f64::powi(b, 2) - 4.0 * a * c))) / (2.0 * a);

    (x1, x2)
}

fn from_x_limits(x1: f64, x2: f64) -> f64 {
    if x1 == (x1 as u32) as f64 && x2 == (x2 as u32) as f64 {
        // Remove 2 since both are ints
        x2.floor() - x1.ceil() + 1.0 - 2.0
    } else {
        x2.floor() - x1.ceil() + 1.0
    }
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
        assert_eq!(test_output, "71503");
        Ok(())
    }

    #[test]
    fn test_quad_form() {
        assert_eq!(get_x_limits(7.0, -1.0, -9.0), (2.0, 5.0))
    }
}
