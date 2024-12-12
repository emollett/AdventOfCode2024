use std::borrow::Cow;
use crate::{Solution, SolutionPair};
use regex::Regex;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = include_str!("input.txt");
    let sol1 = get_result(input);
    let sol2= get_result_2(input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
    re.captures_iter(input).map(|m| (m[1].parse().unwrap(), m[2].parse().unwrap())).collect()
}

fn get_result(input: &str) -> usize {
    let pairs = parse_input(input);
    pairs.into_iter().map(|pair| pair.0 * pair.1).sum()
}

fn remove_dont_blocks(input: &str) -> Cow<str> {
    let re = Regex::new(r"(?ms)don't\(\).*?do\(\)").unwrap();
    re.replace_all(input, "")
}

fn get_result_2(input: &str) -> usize {
    let new_input = remove_dont_blocks(input);
    get_result(&new_input)
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn test_input() -> &'static str {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    }

    #[fixture]
    fn test_input2() -> &'static str {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    }

    #[rstest]
    fn test_parse_input(test_input: &str) {
        assert_eq!(crate::days::day03::parse_input(test_input), (vec![(2,4), (5,5), (11,8), (8,5)]));
    }

    #[rstest]
    fn test_get_result(test_input: &str) {
        assert_eq!(crate::days::day03::get_result(test_input), 161);
    }

    #[rstest]
    fn test_remove_dont_blocks(test_input2: &str) {
        assert_eq!(crate::days::day03::remove_dont_blocks(test_input2), "xmul(2,4)&mul[3,7]!^?mul(8,5))");
    }

    #[rstest]
    fn test_get_result_2(test_input2: &str) {
        assert_eq!(crate::days::day03::get_result_2(test_input2), 48);
    }
}
