use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = include_str!("day01.txt");
    let sol1 = get_distance(input);
    let sol2 = get_frequency_score(input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut locations1 = vec![];
    let mut locations2 = vec![];

    for line in input.lines() {
        println!("{}", line);
        let mut split = line.split_whitespace();
        locations1.push(split.next().expect("can't convert").parse::<usize>().unwrap());
        locations2.push(split.next().expect("can't convert").parse::<usize>().unwrap());
        };

    locations1.sort();
    locations2.sort();

    (locations1, locations2)
    }

fn get_distance(input: &str) -> usize {
    let mut distance = 0;
    let (locations1, locations2) = parse_input(input);
    let compare = locations1.iter().zip(locations2.iter());
    for location in compare {
        let (a, b) = location;
        distance += a.abs_diff(*b);
        };
    distance
    }

fn get_frequency_score(input: &str) -> usize {
        let mut frequency_score = 0;
        let (locations1, locations2) = parse_input(input);
        for location in &locations1 {
            let count = locations2.iter().filter(|a| *a == location).count();
            frequency_score += count * location;
            };

        frequency_score
    }

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn test_input() -> &'static str {
        "3   4
4   3
2   5
1   3
3   9
3   3"
    }

    #[rstest]
    fn test_parse_input(test_input: &str) {
        assert_eq!(parse_input(test_input), (vec![1, 2, 3, 3, 3, 4], vec![3, 3, 3, 4, 5, 9]));
    }

    #[rstest]
    fn test_get_distances(test_input: &str) {
        assert_eq!(get_distance(test_input), 11);
    }

    #[rstest]
    fn test_get_frequency_score(test_input: &str) {
        assert_eq!(get_frequency_score(test_input), 31);
    }
}
