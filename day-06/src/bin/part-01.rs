use std::iter::zip;

fn main() {
    let input = include_str!("./input-01.txt");
    let output = p1(input);
    println!("Solution for part-01: {}", output)
}

fn p1(input: &str) -> String {
    let to_vec_u32 = |s: Option<&str>| {
        s.unwrap()
            .split(':')
            .last()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|val| val.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    };

    let mut lines_it = input.lines();
    let time_vals_it = to_vec_u32(lines_it.next());
    let dist_vals_it = to_vec_u32(lines_it.next());

    let res: f64 = zip(time_vals_it, dist_vals_it)
        .map(|(t, d)| {
            let (b, a, c) = (t as f64, -1_f64, -f64::try_from(d).unwrap());
            let (s, e) = solve_quad_form(b, a, c);

            if s == (s as u32) as f64 && e == (e as u32) as f64 {
                e.floor() - s.ceil() + 1.0 - 2.0 // Remove 2 since both are ints
            } else {
                e.floor() - s.ceil() + 1.0
            }
        })
        .product();

    res.to_string()
}

fn solve_quad_form(b: f64, a: f64, c: f64) -> (f64, f64) {
    let x1 = (-b + (f64::sqrt(f64::powi(b, 2) - 4.0 * a * c))) / (2.0 * a);
    let x2 = (-b - (f64::sqrt(f64::powi(b, 2) - 4.0 * a * c))) / (2.0 * a);

    (x1, x2)
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
        assert_eq!(test_output, "288");
        Ok(())
    }

    #[test]
    fn test_quad_form() {
        assert_eq!(solve_quad_form(7.0, -1.0, -9.0), (2.0, 5.0))
    }
}
